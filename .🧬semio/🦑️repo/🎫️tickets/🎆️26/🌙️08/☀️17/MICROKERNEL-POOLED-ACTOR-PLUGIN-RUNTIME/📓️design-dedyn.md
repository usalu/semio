# Master plan (architecture half): Universal async + zero first-party dyn

Ground truth verified 2026-08-19 against the working tree (staged framework asyncify present).
Target end state: (a) every first-party fn literally `async fn` except language-fixed exceptions,
(b) zero `dyn <FirstPartyTrait>` anywhere, (c) framework + 63 fleet crates compile, tests run.

---

## 0. Rulings (freeze these first — everything downstream cites them)

**R1 — dyn scope.** "Zero first-party trait objects" = no `dyn T` where `T` is one of the 236
first-party traits. `dyn Future`, `dyn Fn/FnMut/FnOnce`, `dyn Any`, `dyn Error` (std/lang traits)
remain PERMITTED, but dyn-Future erasure is confined to (i) argument-position plumbing
(`HostFuture<T>` as `spawn_scoped`'s argument type) and (ii) return types of fn-pointer thunks in
erasure tables (`ComposeFuture`, new `IoFuture`). dyn Future is BANNED from trait-method return
position — that is exactly the double-future damage being removed.

**R2 — async-literal exception classes** (the accepted ~1.9% + two additions the language forces):
- E1: impls of externally-declared traits (serde, Display/Debug, From, Default, Drop, Iterator, Future::poll).
- E2: `const fn`.
- E3: `extern "abi" fn`, `fn main`, proc-macro entry fns.
- E4 (new, must be ratified): fn items whose VALUE is stored in a fn-pointer-typed slot
  (`AsyncComposeFn`, `IoEntry.run/sniff`, `SurfaceDeclaration.factory/app_schema/mutation_roster`,
  `OnceLock<fn()>` installers, `RawWakerVTable`). An `async fn` item's pointer type is unnameable
  (`fn(..) -> <opaque>`), so these CANNOT be async — language-fixed, same class as E3. Discipline:
  E4 fns are either macro-generated (invisible in source) or tagged `// 🚫️async: E4 fn-pointer slot`.
- E5 (new): sync↔async bridge entry points: `block_on`, `LocalExecutor` internals, `poll_ready`/
  `resolve_ready`, hand-rolled `Future::poll` impls (also E1). One per crate at most, tagged `// 🚫️async: E5 executor bridge`.

**R3 — Send boundary.** Guest side (semio-framework-plugin, semio-framework-os-kernel's store guest
paths, all fleet crates): futures are ?Send (single-threaded wasm, `LocalExecutor`,
`⚛️reactor/🧵️executor/🦀️component.rs`). `PluginApp: Send` (type bound) is kept — it constrains the
*state*, not the futures. Host side: Send-ness is obtained STRUCTURALLY, never by bound — every
former `dyn` seam becomes a concrete enum, so at each `spawn` site the future's concrete type is
known and the compiler derives Send. No `+ Send` RPITIT, no return-type-notation, no
`trait-variant`. Rule: if a generic host path ever needs to spawn a trait-method future, the fix is
"route through the enum", never "add a bound". The one erased spawn channel stays
`HostAsyncRuntime::spawn_scoped(&self, scope, ctx, fut: HostFuture<()>)` — callers build the box
with `Box::pin(async move { … })` at concrete types (argument-position dyn Future, R1-legal).

---

## 1. Dispatch redesign per family

### 1.1 GuestRuntime — hand-written enum (closed set)
File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (trait at :512).
```rust
pub enum GuestRuntimes {
    Wasmtime(WasmtimeRuntime),
    // future sibling packet:
    // AsyncActor(WasmtimeAsyncRuntime),
    #[cfg(any(test, feature = "testkit"))] Mock(MockGuestRuntime),
    #[cfg(test)] Recording(shard::RecordingRuntime),   // impl currently at 🧵️shard/🦀️component.rs:1057
}
impl GuestRuntimes {
    pub async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget)
        -> Result<TurnResult, TurnFault>
    { match self { Self::Wasmtime(r) => r.execute_turn(inst, events, budget).await, /* … */ } }
    // same for compile / instantiate / start_job / step_job / cancel_job / checkpoint / restore / drop_instance
}
```
- Trait `GuestRuntime` is KEPT as the AFIT contract the concrete impls satisfy (generic bound only);
  the enum delegates by match. Double-future collapse: `async fn execute_turn(..) -> HostFuture<Result<..>>`
  becomes `async fn execute_turn(..) -> Result<TurnResult, TurnFault>` on trait, impls, and enum.
  Impl bodies drop `Box::pin(std::future::ready(result))` → `result`.
- Call sites: `Arc<dyn GuestRuntime>` → `Arc<GuestRuntimes>` (15 sites: plugin-host, 🧵️shard,
  🧵️shard/🏃️executor.rs, 🌉️mcp/🏠️workspace, renderer wgpu runtime/glue). Construction sites wrap:
  `Arc::new(GuestRuntimes::Wasmtime(WasmtimeRuntime::new(..)))`.
- `poll_ready` (host/🦀️component.rs:~556): callers (`ShardLoop`, `PluginInstanceHandle`, mcp's
  `exchange_one_real`) sit on plain OS threads. Replace poll-once with `semio_framework_async::block_on`
  at the THREAD ROOT (each shard thread runs `block_on(shard_loop.run())`), and plain `.await`
  inside. `poll_ready` itself is demoted to E5, generalized to `pub fn poll_ready<F: Future>(fut: F) -> F::Output`
  and kept only where a sync ABI genuinely cannot block (wasm exports).

### 1.2 HostAsyncRuntime — generics, NOT an enum (layering forces it)
File: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` (:356). Impls live ABOVE the trait's crate
(TokioHostRuntime in `🛎️services/🦀️component.rs:277`, InlineRuntime in db storage :918, ManualRuntime
in-crate testkit :493) — an enum in ⏳️async cannot name them without dragging tokio down (forbidden:
"tokio only in 🛎️services"). Therefore: every `Arc<dyn HostAsyncRuntime>` holder becomes generic.
- Trait repair (double-future unwrap, keep literal async):
```rust
pub trait HostAsyncRuntime: Send + Sync {
    async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle;
    async fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>); // arg-position box stays (R1)
    async fn run_blocking(&self, scope: &ScopeHandle, ctx: OperationContext, work: Box<dyn FnOnce() + Send>);
    async fn sleep_until(&self, deadline_ms: u64);                       // was -> HostFuture<()>
    async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport; // was -> HostFuture<..>
    async fn now_ms(&self) -> u64;
}
```
- Holders (grepped: db 🗄️storage + 🪶️sqlite, 🛎️services, 🔌️plugin/🖥️host/⚡️effects, 📇️directory/🔌️client,
  📺️renderer engine wgpu + Shell element): field `runtime: Arc<dyn HostAsyncRuntime>` →
  `runtime: Arc<R>` with struct param `R: HostAsyncRuntime`. Composition roots pick
  `TokioHostRuntime` (native), `ManualRuntime` (tests), `InlineRuntime` (db-internal scratch).
- Add here: `pub fn block_on<F: Future>(fut: F) -> F::Output` — dependency-free thread-park
  executor (~25 lines, `std::thread::park` waker), tagged E5. This is the universal thread-root and
  test bridge (guest wasm never calls it; guest uses `LocalExecutor`/`resolve_ready`).
- Fix staged damage in this file: `HostFuture` alias survives ONLY as spawn argument currency.

### 1.3 DbStorage family — enum in the db crate (one crate: `semio-framework-os-kernel-db`)
Files: `🛢️db/🗄️storage/🦀️component.rs` (+ 🐘️postgres, 🪶️sqlite, 🌐️neo4j, 🧪️testkit — all modules of the
same crate, so the enum can name every backend).
```rust
pub enum DbBackend<R: HostAsyncRuntime> {
    Memory(MemoryStorage),
    Fs(FsStorage<R>),
    Sqlite(SqliteStorage<R>),
    Postgres(PostgresStorage),
    Neo4j(Neo4jStorage),
    #[cfg(any(test, feature = "testkit"))] Fault(Box<FaultStorage<R>>), // FaultStorage.inner: Arc<dyn DbStorage> → Arc<DbBackend<R>>
}
```
- The 7 traits (WalStorage/SnapshotStorage/PayloadStorage/CatalogStorage/IndexStorage/LeaseStorage/
  DbStorage, :182–:361) stay as AFIT contracts for concrete impls; every method sheds the
  double-future: `async fn append<'a>(&'a self, …) -> DbFuture<'a, u64>` →
  `async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError>`
  (233 `DbFuture` lines; codemod S4). Delete the `DbFuture` alias (:63) once zero uses remain.
- `DbStorage`'s accessor methods returned `&dyn WalStorage` etc. (:362–:367) — replace with
  facet-ref enums with inherent AFIT methods:
```rust
pub enum WalRef<'a, R: HostAsyncRuntime> { Memory(&'a MemoryWal), Fs(&'a FsWal<R>), Sqlite(&'a SqliteWal<R>), Postgres(&'a PostgresWal), Neo4j(&'a Neo4jWal), #[cfg(..)] Fault(&'a FaultWal<R>) }
impl<'a, R: HostAsyncRuntime> WalRef<'a, R> { pub async fn append(&self, …) -> Result<u64, DbError> { match self { … .await } } }
impl<R: HostAsyncRuntime> DbBackend<R> { pub async fn wal(&self) -> WalRef<'_, R> { … } }
```
  (one facet-ref enum per sub-trait; mechanical; call sites `storage.wal().await.append(..).await` —
  keep `wal()` async per decree). `ArtifactEngine.storage: Arc<dyn DbStorage>` (📄️artifact:468) →
  `Arc<DbBackend<R>>`, `ArtifactEngine<R>` generic ripples to `Database::open` which selects the
  variant from the URI at runtime (the contract's runtime selection is the enum's whole point).

### 1.4 Backbone / BackbonePort — enums in store
File: `🏪️store/🦀️component.rs` (traits at :6184/:6190; impls all in-file: PortBackbone :6319,
MemoryBackbone :6351, ChannelBackbone :6396; MemoryBackbonePort :6219, LocalStorageBackbonePort :6252).
```rust
pub enum Backbones { Port(PortBackbone), Memory(MemoryBackbone), Channel(ChannelBackbone) }
pub enum BackbonePorts { Memory(MemoryBackbonePort), LocalStorage(LocalStorageBackbonePort) }
```
- `HOST_BACKBONE_PORT: Mutex<Option<Arc<dyn BackbonePort>>>` (:6196) → `Arc<BackbonePorts>`;
  `set_host_backbone_port(port: Arc<BackbonePorts>)`.
- SDK `attach_backbone(&mut self, backbone: Box<dyn store::Backbone>)` (plugin/🦀️component.rs:9833,
  :12445, :15518) → takes `store::Backbones` by value (no box needed; PortBackbone site :15518
  becomes `store::Backbones::Port(store::PortBackbone::new(uri))`).
- 🪐️space's blanket `impl<T: store::BackbonePort> SpaceBackbonePort for T` (:1333) keeps working
  (generic, and `BackbonePorts` gets a delegating `impl BackbonePort for BackbonePorts` so the
  blanket covers the enum too — trait kept as contract, enum implements it by match; this is the
  ONE family where the enum implements the trait rather than inherent-only, because downstream
  blankets key off the trait).

### 1.5 PluginApp / ArtifactEditor / ArtifactViewer — per-plugin generated enum + generic guest runtime
This is the big one. Verified shape: fleet contains ZERO `dyn PluginApp` — all 26 dyn uses are
inside `semio-framework-plugin` (`🔌️plugin/🦀️component.rs`). Each plugin guest binary is its own
process/component; the erasure is only ever over ONE plugin's own app types (note = 2, trinity ≈ 10,
bounded small). Mechanism:

(a) **`plugin_apps!` decl-macro** in the SDK, defined NEXT TO `trait PluginApp` (:9690) so the
match-delegation arm list co-evolves with the trait in one file. Decl-macro, not proc-macro: the
method list is fixed and SDK-owned, no syn parsing needed; `draw-fsm-macros`
(`✏️s/🔌️plugins/🖍️draw/…/🔄️fsm/✨️macros`, proc-macro crate) stands as precedent/fallback if the
delegation ever needs real parsing.
```rust
// plugin root (e.g. ✏️s/🔌️plugins/🗒️note/🦀️component.rs):
semio_framework_plugin::plugin_apps! {
    pub enum NoteApps {
        editor NoteEditor(crate::editor::note::NotePlayApp),
        viewer NoteViewer(crate::viewer::note::NoteViewer),
    }
}
// expands to:
pub enum NoteApps {
    NoteEditor(VcsArtifactApp<EditorApp<crate::editor::note::NotePlayApp>>),
    NoteViewer(VcsArtifactApp<ViewerApp<crate::viewer::note::NoteViewer>>),
}
impl From<VcsArtifactApp<EditorApp<…NotePlayApp>>> for NoteApps { fn from(x) { Self::NoteEditor(x) } } // E1-sync (From is external)
impl PluginApp for NoteApps {
    async fn app_id(&self) -> &str { match self { Self::NoteEditor(x) => x.app_id().await, Self::NoteViewer(x) => x.app_id().await } }
    /* … one arm-line per PluginApp method, written ONCE inside the macro definition … */
}
```

(b) **Genericize the declaration tree + builder + guest runtime over `A: PluginApp`**
(`🔌️plugin/🦀️component.rs`, `🏗️builder/🦀️component.rs`):
- `SurfaceDeclaration<A>` (:14165): `factory: fn(&AppDefinition) -> A` (bare fn pointer kept —
  same cannot-capture constraint documented at :14158; thunk is E4).
- `editor_surface` (:14175) / `viewer_surface` (:14187):
```rust
pub async fn editor_surface<E: ArtifactEditor, A>(def: AppDefinition) -> SurfaceDeclaration<A>
where A: From<VcsArtifactApp<EditorApp<E>>> {
    fn factory<E: ArtifactEditor, A: From<VcsArtifactApp<EditorApp<E>>>>(def: &AppDefinition) -> A {  // 🚫️async: E4 fn-pointer slot
        A::from(VcsArtifactApp::with_registry(EditorApp::<E>::default(), AppActionRegistry::from_definition(def)))
    }
    fn app_schema<E: ArtifactEditor>() -> Option<AppSchemaDescriptor> { E::app_schema_now() }        // 🚫️async: E4
    SurfaceDeclaration { definition: def, factory: factory::<E, A>, app_schema: app_schema::<E>, mutation_roster: None, rights: Rights::Write }
}
```
- `SubsetDeclaration<A>`, `StandardDeclaration<A>`, `ArtifactDeclaration<A>`,
  `DeclaredRegistration<A>` (:14251: `app_defs: Vec<(App, AppFactory<A>)>` where
  `type AppFactory<A> = (AppDefinition, fn(&AppDefinition) -> A)` — the definition travels WITH the
  pointer, replacing the `Box<dyn Fn() -> Box<dyn PluginApp>>` capturing closure).
- `PluginBuilder<…, A>` / `Plugin<A>` (:12595): `apps: HashMap<String, (AppDefinition, fn(&AppDefinition) -> A)>`;
  `AppInstance<A> { id: u32, app: A }` (:12584); `PluginProgram` (:12593, never dyn-used, single
  impl) becomes `trait PluginProgram { type App: PluginApp; async fn create_app(&self, app_id: &str) -> Option<Self::App>; … }`,
  `impl PluginProgram for Plugin<A> { type App = A; … }`.
- Exchange helpers taking `&mut dyn PluginApp` (:14936/:14943/:14955 —
  `set_merge_policy_frames`/`resolve_conflict_frames`/`read_conflicts_frames`) → generic
  `<A: PluginApp>(app: &mut A, …)`.
- **Guest statics relocate into the export macro.** Today: `thread_local! { static PLUGIN: RefCell<Option<Plugin>> }`
  (:14890) + instance list, consumed by SDK free fns (`plugin_create_app_with_id` :15373,
  `plugin_exchange`, describe path) and the wit export glue anchored by `component_export_anchor`.
  A thread_local cannot be generic, so: introduce `pub struct GuestHost<A: PluginApp> { plugin: Option<Plugin<A>>, assembly_error: Option<Fault>, instances: Vec<AppInstance<A>>, … }`;
  every plugin_runtime free fn becomes `pub async fn plugin_exchange<A: PluginApp>(host: &mut GuestHost<A>, …)`.
  `plugin_exports!` (:16521) grows an `apps` argument and expands the monomorphic layer in the
  PLUGIN crate:
```rust
semio_framework_plugin::plugin_exports!(crate::plugin, apps = NoteApps);
// expands (sketch):
thread_local! { static __SEMIO_GUEST: RefCell<GuestHost<NoteApps>> = …; }
fn __semio_install_plugin_bundle() {                      // 🚫️async: E4 (OnceLock<fn()> slot) — macro-generated
    let plugin = semio_framework_plugin::resolve_ready(crate::plugin()); // builder never truly suspends
    __SEMIO_GUEST.with(|g| g.borrow_mut().install(plugin));
}
/* extern "C" shims (E3) + wit export impls, each delegating into the generic SDK fns with
   __SEMIO_GUEST.with(|g| …) — the wit glue moves OUT of SDK statics INTO this expansion */
```
- `PluginAppMediaFuture` (:9684) is DELETED: with enum dispatch there is no object-safety pressure;
  `export_media`/`media_fingerprint` become plain `async fn … -> Result<Media, MediaError>`. Its
  deliberate not-Send property is now automatic: the AFIT future is ?Send exactly when the state it
  holds is (R3 guest ruling) — and no consumer moves it across threads (guest is single-threaded;
  the only drivers are `.await` inline and `LocalExecutor`).
- Fleet edits: 33 plugin roots add `plugin_apps!` + the `apps =` export arg; 15 subset files
  annotate the enum (usually just the enclosing fn's return type:
  `pub async fn subset() -> SubsetDeclaration<crate::NoteApps>` — `editor_surface::<E, _>` infers).
  Forgetting a variant is a COMPILE error at the subset file (missing `From` impl), so no runtime
  registration drift is possible.

### 1.6 SpaceMember cluster (SpaceMember, ChildStoreFactory, MemberDirectory, LinkResolver) — generics + per-plugin `space_members!` enum  [highest-risk family]
Files: `🏪️store/🦀️component.rs` (SpaceMember :6480 — 25 methods, single blanket impl for
`ArtifactStore<P, Mutation>`; ChildStoreFactory :527 + global `CHILD_STORE_FACTORY_REGISTRY`
:6532-area; SpaceHost :7050 `members: HashMap<String, Box<dyn SpaceMember>>`), SDK
`VcsArtifactApp`'s child map.
- `SpaceHost<M: SpaceMember>` / `CompositionCoordinator<M>` / `Space` structures go generic; all
  `Box<dyn SpaceMember>` → `M`.
- `space_members!` decl-macro exported from the store module (delegation arms written once next to
  the trait, like `plugin_apps!`):
```rust
space_members! { pub enum NoteMembers { Text(ArtifactStore<TextSnapshot, TextMutation>), Sketch(ArtifactStore<…>) } }
// + generated: impl SpaceMember for NoteMembers (match-delegation over the 25 methods);
//              impl MemberFactory for NoteMembers { async fn create(kind: &str, id, dialect, pack) -> Result<Self, VcsError> { match kind { … } } ; async fn open(kind, pack) -> … }
```
- `ChildStoreFactory` + its GLOBAL registry are deleted; the factory becomes the `MemberFactory`
  trait implemented by the generated enum, and the registry keying moves into the enum's own
  `match kind`. The SDK's `VcsArtifactApp<A, M = NoMembers>` takes a second type param with a
  STABLE struct-param default (`pub enum NoMembers {}` — uninhabited, `impl SpaceMember for NoMembers`
  with `match *self {}` bodies), so the many plugins without composition change nothing.
- `as_any_mut(&mut self) -> &mut dyn Any` stays (std trait object, R1-legal) — but with the enum in
  hand, callers can also match directly; keep the method for the existing downcast sites.
- `MemberDirectory`/`LinkResolver` (:452/:446): held-by-value resolvers → generic params on their
  holders (`MemberLinkResolver<D: MemberDirectory>`); both have ≤2 impls, mechanical.
- Fallback if guest/host coupling surprises appear (e.g. a native host harness composing members
  from MANY plugins in one process): the one aggregator crate that already links the whole fleet
  natively generates a fleet-wide `space_members!` enum there. The store never needs to name fleet
  types either way.

### 1.7 Not actually dyn — no redesign needed
The other SDK traits in the 19-trait list (ArtifactSerializer/Deserializer/Composer/Builder/
Decomposer/Analyzer/Analysis/Composition/Children, DerivedArtifactSpec, ArtifactInferrer, LabelAxes,
AppAction, ArtifactApp, WindowKit, ArtifactEditor, ArtifactViewer, PluginProgram) have ZERO dyn
uses (verified by census in SDK + fleet) — plain AFIT is already fine once double-future damage is
unwrapped (§3d). The six registries (ArtifactInferenceServiceRegistry :1084,
ArtifactDefinitionRegistry :2507, HostMediaHandlerRegistry :3452, FlowExtensionRegistry :3523,
PluginRuntimeRegistry :3636, AppActionRegistry :9911) store plain data + fn pointers, not trait
objects — untouched except E4 re-syncing of the fn-pointer targets they hold
(`owner_rosters: &[fn() -> (&'static str, &'static [SemanticDescriptor])]` etc.).

---

## 2. Fn-pointer tables → async (decision: keep fn pointers, macro-generated sync thunks, boxed-future returns)

**Ruling:** `Pin<Box<dyn Future>>` inside the alias returned by a REGISTERED FN POINTER is
dyn-Future erasure in non-trait plumbing — tolerated under R1(ii). An enum over 163 composers
spanning 31 files/33 crates is not viable (open, cross-crate, runtime-registered set); the fn-pointer
table stays, its rows keep `Copy`/`'static`/ptr-equality semantics (`same_io_entry` :2298 relies on
`fn_addr_eq`).

- `AsyncComposeFn = for<'a> fn(&'a [ErasedComposeSource]) -> ComposeFuture<'a>`
  (`🚪️io/🦀️component.rs:751,756`) is KEPT verbatim. The 163 `ComposerEntry` sites currently register
  `async fn compose_hop*` fns that no longer coerce. Fix: new SDK/io decl-macro
```rust
#[macro_export] macro_rules! compose_thunk { ($f:path) => {{
    fn __thunk<'a>(s: &'a [ErasedComposeSource]) -> $crate::ComposeFuture<'a> { Box::pin($f(s)) }  // E4, macro-generated
    __thunk
}}}
```
  and codemod S6 rewrites `compose: some_ident` → `compose: compose_thunk!(some_ident)` in every
  `ComposerEntry { … }` literal. The hop fns themselves KEEP their literal `async fn` (decree
  satisfied; the sync thunk exists only post-expansion).
- `composer_entry_of::<C>()` (SDK :486-area): inner `erased_compose` reverts from `async fn … -> ComposeFuture<'_>`
  (staged damage) to sync `fn` (E4) returning `Box::pin(async move { … C::compose(..).await … })`.
  Same for `serializer_entry_of`/`deserializer_entry_of`.
- `IoEntry` (`🚪️io/🦀️component.rs:2290`): `run`/`sniff` go async-shaped for the end state (real
  suspension is the roadmap):
```rust
pub type IoFuture<'a> = Pin<Box<dyn Future<Output = IoResult<IoPayload>> + Send + 'a>>;
pub struct IoEntry { …, pub sniff: Option<for<'a> fn(&'a IoPayload) -> Pin<Box<dyn Future<Output = Confidence> + Send + 'a>>>, pub run: for<'a> fn(&'a IoPayload) -> IoFuture<'a> }
```
  Registered rows are built exclusively by the SDK's generic `deserializer_entry::<D>()` erasure
  fns (E4 thunks, `Box::pin(async move { D::deserialize(p).await })`). `resolve_ready`
  (`🚪️io/🦀️component.rs:768`) survives ONLY at `wire_artifact_compose`'s sync wasm ABI boundary
  (📌️important.md rule 9) — and its staged-broken raw-waker helpers (`async fn noop`,
  `async fn clone_raw` — currently won't typecheck against `RawWakerVTable`) are replaced outright
  by `std::task::Waker::noop()` (already proven in `poll_ready`).
- `SurfaceDeclaration.factory/app_schema/mutation_roster`, `PLUGIN_BUNDLE_INSTALLER: OnceLock<fn()>`
  (:15037) + `plugin_exports!`'s installer, `owner_rosters: &[fn() -> …]`: E4 re-sync per §1.5.
- FORBIDDEN alternative, stated for the record: "sync wrapper fn per hop, hand-written in source"
  violates the decree's letter; "make ComposerEntry hold a dyn value" violates R1. The macro-thunk
  is the only shape satisfying both.

---

## 3. Repair codemods (specs for a Sonnet executor; all live in the ticket dir next to the two originals)

### S1 `deasyncify-external-impls.py` (fleet, committed damage: 548 Default + 600 serde + 53 From + 31 fmt ≈ 1,232 fns)
Reuse `asyncify-universal.py`'s exact machinery in reverse: same `iter_rs`, same
`collect_local_traits` over `{🧰️framework, ✏️s}` (the 236-trait census), same `IMPL_RE`/`IMPL_FOR_RE`
impl-stack walk. Inside an impl block whose trait's last path segment is NOT in the local census:
rewrite `^(\s*)(pub… )?async fn` → drop the `async `. Emit JSON report {file, line, trait, fn}.
Run `--apply` over `✏️s` only (framework was trait-aware already). Compiler mop-up: any residual
E0053 "method … has an incompatible type" on external impls is fixed by the same span rule.

### S2 `restore-qualifiers.py` (19 `const fn` + 2 `extern` dropped by asyncify-fleet)
Input: the asyncify-fleet commit hash (find via `git log --format='%H %s' -- ✏️s | grep -i asyncif`).
For `git diff <C>^ <C> -U0`, collect hunk pairs where the `-` line matches
`(const |extern "[^"]+" )fn (\w+)` and the `+` line is the same signature as `async fn \2`.
Region-guard: only rewrite the CURRENT file line if it byte-equals the `+` version (else emit a
manual-review row). Restore the original `-` line (const/extern back, async removed — E2/E3).

### S3 `async-test-attr.py` + new proc-macro crate (16,427 sites: 11,553 fleet + 4,704 framework + 170 elsewhere; measured 16,427 in 2,897 files)
New crate `semio-framework-async-macros` at `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust`
(placement per taxonomy: ✨️-sibling proc-macro crate, exactly like `draw-fsm-macros` — "a proc-macro
crate can never be merged into a normal rlib" — and `semio-framework-schema-derive` at
`🧬️schema/✨️derive`). Deps: syn/quote/proc-macro2 (same trio as draw-fsm-macros). One attribute:
```rust
#[async_test]            // also #[async_test(ignore)] passthrough for #[ignore]
async fn my_case() { … }
// expands to:
#[test] fn my_case() { __semio_block_on(my_case_impl()) }     // wrapper: macro-generated sync (E3-adjacent)
async fn my_case_impl() { … }                                  // literal async fn PRESERVED in source? —
```
NO: the SOURCE keeps `#[async_test] async fn my_case()` untouched (that is what the decree's census
greps); the wrapper+rename exists only post-expansion. `__semio_block_on` is EMITTED INLINE by the
macro (self-contained ~25-line thread-park poll loop, hygienic ident) so NO runtime dependency is
added to 65+ crates — only a dev-dependency on the macro crate. Guest-safe: tests run natively
(`cargo test`), never under wasm; no tokio anywhere. `futures_lite::future::block_on`
(already used at `🏃️run/📦️bin.rs:325`) is NOT reused here to avoid fanning futures-lite into every
Cargo.toml; the three crates already using it keep it.
Script: for each file, rewrite `#[test]` (optionally followed by `#[ignore]`) whose next fn line is
`async fn` → `#[semio_framework_async_macros::async_test]` (crates add
`use semio_framework_async_macros::async_test;` — simpler: fully-qualified attr path, zero use-lines);
append `semio-framework-async-macros = { path = "…" }` to `[dev-dependencies]` of the owning
Cargo.toml (path computed by walking up to the nearest 📦️packages/🦀️rust/Cargo.toml).

### S4 `unwrap-double-futures.py` (framework only; fleet has zero `impl Future<Output` — verified)
Patterns (multi-line, brace-matched):
1. `async fn NAME<…>(ARGS) -> DbFuture<'a, T> { Box::pin(async move { BODY }) }` →
   `async fn NAME(ARGS') -> Result<T, DbError> { BODY }` (drop the now-unneeded `'a` binders on args).
   `Box::pin(std::future::ready(EXPR))` bodies → `EXPR`.
2. Same for `HostFuture<T>` (63 lines), `ComposeFuture<…>`, `PluginAppMediaFuture<…>` (6),
   and RPIT `-> impl Future<Output = T> + Send` in the SDK serializer/deserializer/composer trait
   decls (`🔌️plugin/🦀️component.rs:460/:473/:486`): `async fn serialize(from: &Self::From) -> impl Future<Output = Result<…>> + Send`
   → `async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError>`.
~300 sites total (233 DbFuture + 63 HostFuture + rest); script emits every rewrite for review and
refuses ambiguous bodies (multiple `Box::pin`) → manual list. Delete each alias once its use count
hits the R1-permitted residue (HostFuture: spawn arg only; ComposeFuture/IoFuture: thunk returns;
DbFuture/PluginAppMediaFuture: zero → delete).

### S5 `insert-await.py` (the fixpoint loop — the established span-keyed discipline)
Per crate: loop { `cargo check -p <crate> --all-targets --message-format=json`; for each diagnostic:
- E0308 where expected/found note mentions `impl Future`/`opaque type` and a note says
  "consider `await`ing": insert `.await` at the primary span's END byte offset;
- E0599 "no method named X found for opaque type `impl Future`": same insertion at the receiver span end;
- E0277 "`impl Future<…>` is not …" with `?`/iterator notes: same;
apply all edits of one compile pass sorted by (file, offset DESC) so offsets stay valid; never touch
the same (file, span) twice per session (guard set); recompile } until zero insertions happen.
Precedent: db-trait-flip's 4 span-keyed `--message-format=json` scripts. Expected fixpoint depth:
5–15 iterations per crate (one per level of call nesting). Everything the loop cannot fix (genuine
type errors, Send diagnostics) is surfaced as a residue list for hand-fixing — expected residues:
`Result`-vs-future confusions inside closures, futures stored in structs, `map/and_then` chains.

### S6 `compose-thunk-rewrite.py`
In `ComposerEntry { … }` / `IoEntry { … }` literals across ✏️s + 🧰️framework: field `compose:` /
`run:` / `sniff: Some(` whose value is a bare path (not already `compose_thunk!`): wrap in the
matching thunk macro. 163 ComposerEntry sites + IoEntry rows. Idempotent (skips wrapped sites).

---

## 4. Sequencing to green (gate ladder; keep the staged framework asyncify — commit it as the branch's first commit so every later diff reviews cleanly)

Compile-order spine: semio-framework-async → semio-framework (io/pack/schema/machine modules) →
semio-framework-os-kernel (store) → semio-framework-os-kernel-db → semio-framework-plugin (SDK) →
semio-framework-plugin-host → services/mcp/renderer/run/os product → 🗄️stdio → fleet batches.

| # | Step | Parallel? | Acceptance |
|---|------|-----------|------------|
| 0 | Commit staged asyncify; ratify R1–R3 + E4/E5 in the ticket's 📌️important.md; create `semio-framework-async-macros` crate + `block_on` in ⏳️async | – | `cargo check -p semio-framework-async-macros -p semio-framework-async` |
| 1 | ⏳️async: HostAsyncRuntime de-double-future (S4), Waker::noop fixes, ManualRuntime | – | `cargo check -p semio-framework-async --all-features` + S5 loop |
| 2 | 🚪️io (inside semio-framework): IoFuture, IoEntry async-shaped, resolve_ready raw-waker fix, compose_thunk!, composer/serializer erasure fns → E4 | with 3 | `cargo check -p semio-framework` + S5 |
| 3 | 🏪️store (kernel crate): Backbones/BackbonePorts enums, SpaceMember genericization + space_members!/NoMembers, ChildStoreFactory deletion | with 2 | `cargo check -p semio-framework-os-kernel` + S5 |
| 4 | 🛢️db: DbBackend + facet-ref enums, DbFuture unwrap (S4), HostAsyncRuntime generics | after 1 | `cargo check -p semio-framework-os-kernel-db --all-features` + S5 |
| 5 | SDK (ATOMIC, one crate): §1.5 in full — trait double-future unwrap, declaration-tree generics, plugin_apps!, plugin_exports! rework, GuestHost, exchange generics, PluginAppMediaFuture deletion | – | **gate 1: `cargo check -p semio-framework-plugin --lib` (zero E0038)** |
| 6 | plugin-host: GuestRuntimes enum, block_on at shard-thread roots, poll_ready demotion | after 5 | **gate 2: `cargo check -p semio-framework-plugin-host --all-targets`** |
| 7 | services/kernel/mcp/renderer/run/os-product: Arc<GuestRuntimes>, HostAsyncRuntime generics ripple, S5 loops | crates in parallel | **gate 3: `cargo check -p <os product> --all-targets`** |
| 8 | Framework tests: S3 over 🧰️framework (4,704), S5 over --tests | – | `cargo test -p semio-framework-async -p semio-framework -p semio-framework-plugin …` ladder |
| 9 | Fleet offline codemods, whole fleet at once (no compile needed): S1, S2, S3, S6 | fully parallel | S1/S2 JSON reports reviewed; `git diff --stat` sanity |
| 10 | 🗄️stdio (every plugin depends on it): plugin_apps! root, exports arg, subset annotations, S5 loop | – | **gate 4: `cargo check -p semio-s-plugin-stdio --all-targets` then `cargo test -p semio-s-plugin-stdio`** |
| 11 | Remaining 32 plugins + 26 extension crates in parallel batches of ~6 (independent crates; per-crate S5 loops; roots/subsets mechanical edits per §1.5) | batches parallel | per crate: check → test |
| 12 | Global sweep | – | `cargo check --workspace --all-targets`; `cargo test --workspace`; census: `dyn <any of 236 first-party traits>` = 0; async-literal census ≥ 98.1% |

**E0053/E0038 interplay — what still breaks in the fleet after de-dyn (quantified):** the fleet's
56,680 `async fn` impl bodies now MATCH the AFIT traits, so de-dyn produces no new fleet signature
errors. Remaining fleet red is exactly: (i) ~1,232 external-impl E0053s (S1), (ii) 19+2 E2/E3
qualifiers (S2), (iii) 11,553 "async fn cannot be used for tests" errors (S3), (iv) missing
`.await`s — tens of thousands, all S5-driven, (v) 163 fn-pointer coercion E0308s (S6),
(vi) 33 roots + 15 subset files of hand edits (§1.5), (vii) Send: nothing new — guest futures are
?Send by ruling R3 and no fleet consumer spawns across threads.

**Risk register:** SpaceMember genericization (§1.6) is the highest-risk item — if a host-native
multi-plugin composition path surfaces, use the aggregator-crate fleet enum fallback. Second risk:
decl-macro `plugin_apps!` arm list drifting from `trait PluginApp` — mitigated by colocation in one
file + a testkit assertion that the enum impl compiles against every method (any drift is a compile
error by construction). Third: S5 loop convergence on the two giant files (SDK 20,733 lines) —
budget manual passes there.
