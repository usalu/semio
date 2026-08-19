# 📓️ terra — sdk-dedyn report

Packet: **sdk-dedyn**. Crate `semio-framework-plugin` (guest SDK), owned path
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` excluding `🖥️host/**`. ATOMIC (R6).

**Bottom line: Steps 1 and the core of Step 2 are done and internally consistent. One large,
well-scoped piece of Step 2 (the `plugin_runtime` guest-statics module) is explicitly NOT
converted — STOPPED and reported per the brief's own STOP condition, with the exact reason. A
second, unrelated family (`SpaceMember`/`children` inside `VcsArtifactApp`) is also STOPPED for a
genuine architectural-mismatch reason discovered while applying store-dedyn's lease. Acceptance is
UNRUN: the crate still cannot be reached by rustc — blocked upstream, but by a DIFFERENT and larger
crate than the brief anticipated (see "Acceptance" below).**

---

## 1. Step 1 — re-asyncify + E4 fixes

### Before/after counts (python3 over the absolute path, not grep)

Verified fresh from disk immediately before running the codemod:

```
Main SDK file (🔌️plugin/🦀️component.rs): 19 async fn in the working tree, 1,489 in git index
Host file (🔌️plugin/🖥️host/🦀️component.rs, NOT mine): 1 async fn (untouched, confirmed after)
```

Ran `asyncify-universal.py --scan` then `--apply` over every owned subtree EXCLUDING `🖥️host/**`
(`⚛️reactor`, `🌐host`, `🏗️builder`, `📇️describe`, `📇️registry`, `📦️packages`, `🛂️describe`,
`🦀️component.rs`, `🧬️schema`, `🪟️window-kits` — 19 files):

```
local traits known: 241 | files: 19
{ "converted": 1773, "const": 3, "extern": 6, "external_trait": 52, "main": 1, "already": 47, "tagged_exempt": 0 }
```

After: main SDK file **1,489 `async fn`** — exact match to the git index count. Brace count
5,224/5,224 both before and after (structurally sound). Host file unchanged at **1**. Scan/apply
logs: `terra-sdkdedyn-scan1.txt`, `terra-sdkdedyn-apply1.txt` in this ticket folder.

### The three named E4 files

1. **`⚛️reactor/🧵️executor/🦀️component.rs`** — reverted `raw_waker`/`waker_clone`/`waker_wake`/
   `waker_wake_by_ref`/`waker_drop` to plain `fn`/`unsafe fn`, each tagged `// 🚫️async: E4
   fn-pointer slot`, exactly matching the probe fixture at
   `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🌐️native/🦀️src/executor_patched.rs` (read
   before editing, per the brief). Added the 2 missing `.await`s in `run_until_idle`
   (`self.waker_for(id).await`, `self.has_pending().await`). Then fixed the file's own 7
   `#[cfg(test)]` fns, which were still calling the now-async `LocalExecutor` methods
   (`new`/`spawn`/`cancel`/`run_until_idle`/`has_ready`/`has_pending`) without `.await` — added
   `.await` at every call site (this file's tests are small and self-contained, so I hand-verified
   each one rather than leaving it for a tool).
2. **`⚛️reactor/📮️requests/🦀️component.rs`** — `futures_test_waker`/`noop`/`clone`'s hand-rolled
   `RawWakerVTable` replaced OUTRIGHT by `std::task::Waker::noop()` (per the brief: "where
   possible" — here it is, since the waker never needs real wake behaviour in tests), tagged E4.
   Fixed the 5 call sites' `Context::from_waker(&waker)` → `Context::from_waker(waker)` (the
   function now returns `&'static Waker` directly, not an owned value). Then fixed the 5 test
   fns' ~39 missing `.await`s on `RequestRegistry::new/request/drain/resolve/append_chunk/
   pending_ids/for_instance/cancel_instance` via a small brace-matching python script (paren-depth
   tracking, not naive regex, since several calls have nested closures) — verified by reading the
   diff before/after.
3. **`🌐host/📖️body/🦀️component.rs`** — the test-only `block_on` helper was ALSO wrongly
   asyncified (its whole purpose is to be the sync/async bridge test bodies call into — an `async
   fn` can never be that). Reverted to plain `fn`, its hand-rolled vtable replaced by
   `Waker::noop()`, tagged `// 🚫️async: E5 executor bridge (test-only, R4 clause 5)`. `next_chunk`/
   `collect`/`poll_buffered`/`direct` stay genuinely `async fn` — unchanged (already correct).

### Additional E4 sites found and fixed while doing Step 2 (not in the original checklist, but the
same defect class, found by reading, not guessed)

- `check_non_negative` (fixture `Deserializer::CONFORMANCE: Option<fn(&T) -> Vec<Diagnostic>>`) —
  reverted to sync `fn`, tagged.
- `composer_entry_of<C>`/`deserializer_entry_of<D>`/`serializer_entry_of<S>` — these build a
  `ComposerEntry` value inside `std::sync::Once::call_once`'s FIXED-SYNC closure (the `subset!`
  macro's `__subset_registration::register`), so per **R9** they must stay sync. Reverted to plain
  `fn`, tagged `// 🚫️async: E1 pure struct-builder consumed by std::sync::Once::call_once`. Their
  inner `erased_compose` helpers (the actual `AsyncComposeFn` bare-fn-pointer values) reverted to
  plain `fn` returning `Box::pin(async move {...})`, tagged E4 — exactly the shape design-dedyn.md
  §2 specifies.
- `PLUGIN_BUNDLE_INSTALLER`/`EXTENSION_BUNDLE_INSTALLER: OnceLock<fn()>` — tagged at the static
  declaration.
- `__semio_install_plugin_bundle`/`__semio_install_extension_bundle` (inside `plugin_exports!`/
  `extension_exports!`) — these were `async fn` items being handed as VALUES into
  `register_plugin_bundle_installer(install: fn())` (a bare-fn-pointer parameter) — an async fn
  item's pointer type is unnameable, so this could never have coerced. Reverted to plain `fn`,
  bridged to the still-genuinely-async `install_plugin_bundle_result`/`install_extension_bundle`
  via `resolve_ready` (`$crate::app::resolve_ready`, since the macro expands inside the CONSUMING
  plugin crate). Tagged E4.

### S4 — double-future removal

`ArtifactSerializer::serialize`/`ArtifactDeserializer::deserialize`/`ArtifactComposer::compose`
declared `async fn X(...) -> impl Future<Output = Result<..>> + Send` — the exact double-future
shape R1 bans (their own real impls, e.g. `DummySerializer`, already implemented the CORRECTED
single-future signature, so the trait declaration was the thing out of sync, not the impls).
Fixed all three to plain `async fn X(...) -> Result<..>`.

### `async-test-attr.py`

Ran over the same 19-file scope (the tool's `Path.rglob("*.rs")` silently returns nothing when a
root is a **file** rather than a directory — worked around by importing the module directly and
calling `process_file` on `🔌️plugin/🦀️component.rs` explicitly, since it's a bare file, not a
dir). **293 sites across 12 files** rewritten `#[test]` → `#[semio_framework_async_macros::
async_test]`; 2 `Cargo.toml`s (`🔌️plugin/📦️packages/🦀️rust`, `🔌️plugin/📇️describe/📦️packages/🦀️rust`)
got the `[dev-dependencies]` path entry. Idempotent, verified by re-running `--scan`-equivalent
logic after.

---

## 2. Step 2 — de-dyn `PluginApp` (design-dedyn.md §1.5)

### Mechanism used

The shared macro, per the coordinator's explicit override of the design doc's older `plugin_apps!`
sketch: `use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};` added to `app` module's
imports; `#[dyn_enum]` on `pub trait PluginApp: Send { .. }`. Added
`semio-framework-dispatch-macros` as a `[dependencies]` line in `🔌️plugin/📦️packages/🦀️rust/
Cargo.toml` (my own crate's manifest — NOT the registrar-only root) and `#![allow
(async_fn_in_trait)]` at the crate root (`📦️glue.rs`), per R7/the macro's own recipe.

Verified against the macro's four documented rejection conditions (finding 4, recipe §1 in
`terra-dyn-enum-macro-report.md`) by careful reading, since I could not get a real compile through
this trait (see "Acceptance"): `PluginApp` has no associated types/consts, no method with no
`self` receiver, no destructuring parameter pattern, and no mixing of `self: Arc<Self>` with `&mut
self` (every method is `&self` or `&mut self` only) — should be accepted by `#[dyn_enum]` cleanly.
**This is the one place I could not get a real rustc verdict and am reporting a careful-reading
conclusion instead of a compiler one** (see §4).

`PluginAppMediaFuture` (the manual `Pin<Box<dyn Future<..>>>` workaround for the pre-existing
object-safety pressure) **deleted**. `export_media`/`media_fingerprint` (trait default bodies AND
`VcsArtifactApp<A>`'s own impl) converted to plain `async fn -> Result<T, MediaError>`. While in
that exact code I also fixed `produce_media`/`consume_media`'s pre-existing missing `.await`s
(`self.document_pack()`, `self.document_schema()`, `self.load_document_pack(..)`) and a genuine
bug the async conversion introduced: `MediaWireFormat::Document { schema } if schema ==
self.document_schema()` — **a match GUARD can never `.await`** — resolved into a local
`schema_now` computed before the `match` instead.

### The default: `NoPluginApp`

`dyn_enum_close! { pub enum NoPluginApp: PluginApp {} }` — a zero-variant enum, same pattern as
store-dedyn's `NoMembers`/`NoBlobStore`/`BackboneChannelPorts` (design-dedyn.md §1.6's own
precedent), immediately after the trait. Every method's body degenerates to `match *self {}`
(macro-generated, verified against the macro's own "uninhabited enum" test coverage — I did not
hand-write 38 match arms). Set as the DEFAULT type parameter (`PA: PluginApp = NoPluginApp`) on
every generic in the declaration tree AND on `Plugin`/`AppInstance`/`PluginBuilder`. **Caveat I
want to be explicit about**: Rust struct-level defaults only resolve when the type name is written
elided (`let x: Plugin = ..`); they do **not** rescue plain inference (`Plugin::builder("id")` with
no type ever named anywhere still needs an explicit `Plugin::<SomeType>::builder(..)`) — I
initially assumed otherwise and had to correct course. Applied explicit turbofish at every
test call site that had no other way to pin `PA` down (see §2.4).

### Declaration tree — fully converted, generic over `PA: PluginApp`

`SurfaceDeclaration<PA>` (factory: `fn(&AppDefinition) -> PA`, E4-tagged, same bare-fn-pointer
shape as before minus the `Box<dyn PluginApp>`), `editor_surface<E, PA: From<VcsArtifactApp
<EditorApp<E>>>>`/`viewer_surface<V, PA: From<VcsArtifactApp<ViewerApp<V>>>>` (inner `factory`/
`app_schema` thunks reverted sync + E4-tagged, `app_schema` bridged via `resolve_ready` since
`ArtifactEditor::app_schema()` is a genuine pure AFIT method), `SubsetDeclaration<PA>`,
`StandardDeclaration<PA>`, `ArtifactDeclaration<PA>` (careful: distinct from the OLD, unrelated
`app::ArtifactDeclaration` two regions up — did not touch that one), `DeclaredRegistration<PA>` +
new `type AppFactory<PA> = (AppDefinition, fn(&AppDefinition) -> PA)` (definition travels WITH the
bare pointer, replacing the old `Box<dyn Fn() -> Box<dyn PluginApp> + Send>` capturing closure).
`format_descriptor_of`/`capability_rows_for`/`check_surface_id`/`preflight_artifact_declarations`/
`preflight_io_entries`/`commit_artifact_declarations` all threaded `<PA: PluginApp>` through, and
— since I was rewriting these functions anyway — fixed their own extensive PRE-EXISTING missing-
`.await` damage (`format_descriptor_of(..)` was being pushed un-awaited into a `Vec`,
`check_surface_id(..)?` was applying `?` to a `Future`, `preflight_artifact_inference_services`/
`dsl::preflight_languages`/6 more registry calls were un-awaited) — these were genuine, separate,
pre-existing bugs from the blind codemod, not something the PA generic caused, but I fixed them
because I was already rewriting the exact lines.

The `#[cfg(test)] mod fixture` (a synthetic 2-standard/3-subset artifact, W1-C's own executable
spec) now needs a concrete `PA` — gave it its own `FixtureApps` enum via a cross-module
`dyn_enum_close!` call (`use crate::app::__semio_dispatch_PluginApp;` immediately above, per
finding 1's cross-module requirement), 6 variants (editor+viewer × 3 subsets). Fixed the fixture's
own extensive missing-`.await`s (`Editor::builder(..).document(..).mode(..).window_kind(..)
.build_definition()` — every step of that builder chain is async; `.encode_pack()`/`.decode_pack()`
are default-bodied `async fn`s on `ArtifactPack`, not sync convenience methods) in the functions I
was directly rewriting for the `PA` threading (`native_codecs`, `editor_definition`,
`viewer_definition`, `build_declaration`, `std1_strict_entries` — reverted to sync + `resolve_ready`
since it's called from `OnceLock::get_or_init`'s fixed-sync closure — and the whole `Tests` block).
`testkit::{assert_declaration_tree_registers_all, assert_declaration_registration_is_atomic,
assert_subset_declaration_ids_are_derived}` genericized to `<PA: super::PluginApp>` and their own
missing awaits fixed for the same reason.

### `Plugin`/`AppInstance`/`PluginProgram`/`PluginBuilder` — fully converted

- `AppInstance<PA: PluginApp = NoPluginApp> { id: u32, app: PA }` (was `Box<dyn PluginApp>`).
- `PluginProgram` gained `type App: PluginApp;`, `create_app` returns `Option<Self::App>`. Verified
  it is never itself `dyn`-used anywhere (repo grep), so it stays a plain AFIT contract, not a
  second `#[dyn_enum]` family.
- `Plugin<PA: PluginApp = NoPluginApp>`: `apps: HashMap<String, (App, AppFactory<PA>)>` — wait,
  precisely `HashMap<String, AppFactory<PA>>`, `App`'s manifest metadata pushed separately as
  before. `register_app_factory` takes `AppFactory<PA>` directly (no more `impl Fn() -> Box<dyn
  PluginApp>`). `create_app` calls `factory(definition)`.
- `PluginBuilder<State, PA: PluginApp = crate::app::NoPluginApp>` (`🏗️builder/🦀️component.rs`, 964
  lines, fully converted): every typestate transition (`new`/`label`/`version`) threads `PA`
  through unchanged (mechanical field-copy). `document_app::<A>`/`viewer::<V>`/`editor::<E>` each
  had a CAPTURING closure factory (`Box::new(move || Box::new(VcsArtifactApp::with_registry(...,
  registry.clone())))`) — converted to the same bare-fn-pointer + `AppFactory<PA>` shape the
  declaration tree uses (`fn factory<E, PA: PluginApp + From<VcsArtifactApp<EditorApp<E>>>>(def:
  &AppDefinition) -> PA { PA::from(VcsArtifactApp::with_registry(EditorApp::<E>::default(),
  AppActionRegistry::from_definition(def))) }`, E4-tagged), rebuilding the registry from `def`
  inside the fn body instead of capturing it — same trick `editor_surface` already used.
  `try_build`/`try_library` return `Result<Plugin<PA>, ..>`. Fixed this function's own extensive
  missing-`.await` chain while rewriting it (`Plugin::new(..).with_runtime_registry(..)`,
  `contribution.resolve(..)`, `register_contributions`, `extend_contributions`,
  `store::begin_artifact_assembly()`, `install_plugin_descriptor_extras`, and the OLD (non-generic)
  `ArtifactDeclaration::apply_to` — whose body did `self.capabilities.into_iter().fold(plugin, |p,
  c| p.capability(c))`, which cannot `.await` inside a plain `Iterator::fold` closure; restructured
  to a for loop).
- The 3 exchange helpers `set_merge_policy_frames`/`resolve_conflict_frames`/
  `read_conflicts_frames` (used by the channel-command dispatch path) genericized
  `<PA: PluginApp>(app: &mut PA, ..)` / `(app: &PA, ..)`, replacing `&mut dyn PluginApp`/`&dyn
  PluginApp`, and their own missing `.await`s fixed.

### Type-inference fallout from the closures → bare-fn-pointer conversion

`PluginBuilder`'s own two test modules (`plugin_builder_dependency_tests`,
`schema_stamping_tests`) construct `Plugin::builder("id")....try_build()` chains that never
name a concrete `PA` anywhere. Once `Plugin`/`PluginBuilder` became generic, these went from
"trivially infers `Box<dyn PluginApp>`" to "cannot infer `PA`" (defaults don't rescue pure
inference, see above). Fixed: the 6 dependency-gating/host-media/flow-extension tests (none call
`.editor`/`.viewer`/`.document_app`) → `Plugin::<crate::app::NoPluginApp>::builder(..)`. The 3
schema-stamping tests DO call `.editor::<SchemaStampEditorFixture>`/`.viewer::<
SchemaStampViewerFixture>`, which need a real `From<VcsArtifactApp<..>>` impl `NoPluginApp`
(uninhabited) cannot provide — gave that test module its own small `SchemaStampApps` enum
(2 variants) via the same cross-module `dyn_enum_close!` recipe. **I did not chase these tests'
OWN pre-existing missing-`.await` chains beyond making the `PA` type-correct** (documented as
residual, S5-territory, in §5).

### Verification (what I *could* do without a compiler)

```
$ grep -n "dyn PluginApp" 🔌️plugin/🦀️component.rs | grep -vE ':\s*(///|//)'
(zero lines — every remaining "dyn PluginApp" is inside a doc comment)

$ python3 -c "brace/paren balance check on both files"
🔌️plugin/🦀️component.rs:            braces 5228/5228, parens 16841/16841
🔌️plugin/🏗️builder/🦀️component.rs:  braces 156/156,   parens 614/614
```

`async fn` count in the main file: **1,473** (down from 1,489 — the 16-fn delta matches the E1/E4
reversions listed above; I did not lose track of any).

---

## 3. STOP items — explicit, per the brief's own STOP condition

### 3a. `plugin_runtime` module — NOT converted (the single biggest open item)

This module (`pub mod plugin_runtime { .. }`, guest WIT-export glue) is **far larger than design-
dedyn.md §1.5(b)'s own framing suggested** — not "33 plugin roots + 15 subset files" (that's the
FLEET side); on the SDK side alone it is close to **4,300 lines and ~56 `async fn`s**, all built
around FOUR `thread_local!` statics (`PLUGIN: RefCell<Option<Plugin>>`, `PLUGIN_ASSEMBLY_ERROR`,
`INSTANCES: RefCell<Vec<AppInstance>>`, `INSTANCE_ACTORS`) that every one of those functions reads
through a `.with(|slot| ..)` closure.

Two independent facts, found by reading, drove the decision to stop rather than push a blind
rewrite:

1. **A `thread_local!` cannot be generic.** Design-dedyn.md §1.5(b) itself says the fix is to
   relocate the statics into a `GuestHost<PA>` struct that only exists once `plugin_exports!`
   monomorphizes it per CONSUMING plugin crate — i.e. this is not "add `<PA: PluginApp>` to each
   function signature", it is "rewrite every function from implicit-thread-local access to
   explicit `&GuestHost<PA>`/`&mut GuestHost<PA>` parameter-passing", across all ~56 functions and
   every one of their call sites (including inside the `plugin_exports!`/`extension_exports!`
   macro bodies, which generate the WIT `extern "C"` shims).
2. **The module already has extensive, PRE-EXISTING async-inside-fixed-sync-closure damage from
   the blind codemod**, independent of anything I did: e.g. `plugin_wire_list_artifact_inference_
   services` calls `plugin.wire_list_artifact_inference_services()` (now `async fn`) UN-AWAITED
   inside `PLUGIN.with(|slot| {..})`'s closure — `RefCell`'s / `LocalKey::with`'s closure parameter
   is a FIXED sync `FnOnce`, so this cannot simply get a `.await` added; it needs either the
   accessor demoted to sync (R9, if it's genuinely pure — needs per-function judgement) or the
   whole call restructured to extract the future outside the closure and await it after. This
   pattern recurs across roughly 20+ of the ~56 functions I read.

Doing BOTH the generic-parameter threading AND correctly untangling the pre-existing sync-closure/
async mismatch, by hand, across ~4,300 lines, with **zero compiler feedback available** (see §4),
is exactly the situation my brief's STOP condition describes ("If the generic parameter proves to
thread through far more public types than the design anticipated, STOP and report... that is my
call to make, not yours"). I stopped here rather than risk introducing many more subtle, unverified
type/borrow errors into the module every fleet plugin's guest runtime depends on.

**What IS done, so this isn't a flat "nothing happened"**: the type-level foundation this module
would need to build against is complete and internally consistent — `Plugin<PA>`, `AppInstance<PA>`,
the declaration tree, `PluginBuilder<State, PA>` all exist, compile-shape-correctly reference each
other, and default to `NoPluginApp`. What's missing is exactly the ~56-function/~4,300-line
`GuestHost<PA>` restructuring of `plugin_runtime` itself, which currently still names the bare
(now-invalid, missing-generics) `Plugin`/`AppInstance` and will not compile until that lands.
Concretely broken right now (confirmed by direct reading, not the blocked compiler): every
`PLUGIN.with(..)`/`INSTANCES.with(..)` call site, `instance.app.as_ref()` (only 1 occurrence — was
`Box<dyn PluginApp>::as_ref()`, now meaningless since `app: PA` has no `Box`), and every free fn
whose signature bare-names `Plugin`/`AppInstance`.

**Recommendation**: this is large enough to be its own follow-up packet (something like
`sdk-guest-host`), gated on `semio-framework-os-kernel` actually reaching green so the S5
`insert-await.py` fixpoint loop can run against it with real diagnostics — attempting it blind, as
I've now discovered by reading the whole module, would very likely need a second attempt anyway.

### 3b. `SpaceMember`/`children` inside `VcsArtifactApp` — NOT converted (genuine design gap found)

store-dedyn's lease-request #2 (`open_child`) asks for `M::open(kind_str, envelope_pack)` replacing
the now-deleted `store::child_store_factory`, via a `VcsArtifactApp<A, M = store::NoMembers>`
second type parameter. I read the actual landed `CompositionCoordinator::dispatch_group`/
`dispatch_peer_group` signatures in `🏪️store/🦀️component.rs` before attempting this, and found a
real mismatch the lease's author did not have a chance to verify (their own report says they never
got a compiler check past `semio-framework-hash`/`-dsl-derive`/`-replication`):

```rust
pub async fn dispatch_group<M: SpaceMember + MemberFactory>(
    &mut self, parent_ref: &ArtifactRef, parent: &mut M, children: &mut [(&mut M, ChildDispatch)], ..
) -> Result<GroupReceipt<M>, VcsError>
```

**`parent` and every entry in `children` must be the SAME concrete type `M`.** But
`VcsArtifactApp<A>`'s parent (`self.store: ArtifactStore<A::Snapshot, A::Mutation>`) is a FIXED,
app-specific type, while `M` is meant to be the per-plugin CHILD-composition enum (heterogeneous
artifact kinds a document composes) — these are structurally different things. The CURRENT code
works around this today by casting both to `&mut dyn SpaceMember` (`&mut self.store as &mut dyn
SpaceMember`), which is exactly the erasure the whole ticket removes. Under pure generics there is
no common type unless the plugin's own `M` enum ALSO enumerates a variant for the parent's own
concrete type — which `space_members!`'s documented shape (child kinds only) does not do, and
which the SDK cannot invent on its own without a real design decision.

This reaches into `VcsArtifactApp.children: HashMap<(String,String), (ArtifactDialect, Box<dyn
SpaceMember>)>`, `open_child`, `register_child`, `absorb_created_children`, `child_store`, and
`dispatch_group`'s own raw-pointer multi-mutable-borrow dispatch (`*mut dyn SpaceMember` →
`*mut M`) — unsafe code I am not willing to hand-edit without a compiler double-checking it, on
top of the type mismatch above. Also: `Box<dyn SpaceMember>` is independently broken right now
regardless of `child_store_factory` (SpaceMember's own methods are all `async fn` post-store-dedyn,
so `Box<dyn SpaceMember>` fails E0038 at the type-declaration site, before any call is even made) —
this is a real, pre-existing break in my owned file, NOT something I introduced.

**I left this code exactly as store-dedyn's landed state left it** (still referencing the deleted
`child_store_factory`, still declaring `Box<dyn SpaceMember>`) rather than attempt an unverified
fix to unsafe pointer code. **Recommendation**: this needs the coordinator's design call — either
(a) `space_members!`-generated enums grow a "self" variant for the composing document's own type,
or (b) `VcsArtifactApp`'s composition machinery gets its own dedicated packet alongside the
`SpaceMember` cluster design-dedyn.md §1.6 already flags as "highest-risk item".

### The `attach_backbone` half of store-dedyn's lease — DONE

`attach_backbone(&mut self, backbone: Box<dyn store::Backbone>)` at the trait method and
`VcsArtifactApp`'s impl → `attach_backbone(&mut self, backbone: store::Backbones)`, `self.store
.attach_backbone(backbone).await` (store's own method is now async). `plugin_attach_backbone`'s
`Box::new(store::PortBackbone::new(uri))` → `store::Backbones::Port(store::PortBackbone::new(uri))`.
8 test call sites `Box::new(near)` → `near.into()` (the generated `From<Variant> for Backbones`).

### io-thunks' lease — DONE, in the form that actually matches the current tree

The lease's cited line (`commit_artifact_registration_plan`) was, by the time I reached it, ALREADY
`async fn` (my own Step-1 codemod converted it — it wasn't tagged, and it wasn't in an
externally-declared-trait impl). Rather than reverting it to sync + `resolve_ready` (the lease's
primary suggestion), I took the lease's own documented "alternative": added the missing `.await`
inside its body (`semio_framework::io::commit_artifact_assembly_registry_plan(assembly, plan)
.await.map_err(..)`) and `.await`ed its one call site in `🏗️builder/🦀️component.rs`'s `try_build`.

---

## 4. Acceptance — UNRUN, blocked upstream, but by a DIFFERENT crate than briefed

```
$ CARGO_TARGET_DIR=<scratchpad>/target-sdk cargo check -p semio-framework-plugin --lib
```
Run in the foreground, one turn, no timeout truncation (completed on its own before the 120s
auto-background threshold on the FIRST attempt at the start of this packet; on the SECOND attempt,
after all edits, it ran to completion within the 600s explicit timeout). **Two runs, two different
outcomes** — both pasted, both exit code and message-format=json cross-checked:

**Run 1 (before any edits, to establish the baseline)**: stopped at `semio-framework-pack`, 1
error (`E0502` borrow conflict in a CRC helper — the exact bug status.md's tail described
`pack-finish` as actively fixing). Matches the brief exactly.

**Run 2 (after all edits above)**: `semio-framework-pack` is now GREEN (no longer appears as an
error source at all) — `pack-finish` finished while I was working. The build proceeds further and
now stops at **`semio-framework-os-kernel`: exit 101, 1,258 errors**. Cross-checked with
`--message-format=json`, grouped by crate and by file:

```
=== errors by crate ===
1258 semio_framework_os_kernel
=== errors by file (top 10) ===
339  🏪️store/🦀️component.rs
184  🗣️dsl/🧬️schema/🦀️component.rs
177  📡️spr/📜️history/🦀️component.rs
103  🗣️dsl/📖️grammar/🦀️component.rs
65   📡️spr/🧪️testkit/🦀️component.rs
58   🚪️io/🦀️component.rs
44   🎒️pack/🧪️testkit/🦀️component.rs
29   🗣️dsl/🖋️notation/🦀️component.rs
26   🗣️dsl/🦀️component.rs
24   🎒️pack/🔢️value/🦀️component.rs
```

**Zero of the 1,258 errors are attributed to any file under my owned path (`🔌️plugin/**`).** The
compiler never reaches my crate's own source at all — it is still blocked one layer below where
the brief expected. Sampled the error shapes: several are the SAME class of bug I've been fixing
in my own file (`PayloadSource`/`ResourceResolver`/`PayloadSink` traits E0038-broken because their
async methods are stored behind `dyn`/`Box<dyn ..>`; match-guard `.await` illegality; `Iterator`/
`fold`-closure async mismatches; recursive-async-fn-needs-`Box::pin` E0733s; missing `.await`s
throughout `🏪️store`/`🗣️dsl`/`📡️spr`) — none of it is mine to fix (all outside `🔌️plugin/**`,
per rule 3), and I did not touch any of it.

**Therefore, per my brief's own instruction ("report your work as UNRUN with the blocking crate
named... do NOT edit to unblock yourself"), I report**: `cargo check -p semio-framework-plugin
--lib` and `--all-targets`, and `cargo test -p semio-framework-plugin --lib`, are **UNRUN** —
blocked by `semio-framework-os-kernel` (1,258 errors, 0 attributable to my owned path), not by
`semio-framework-pack` (now green). This is a materially different, and larger, blocker than the
brief anticipated; flagging the crate-name change explicitly since "still blocked by pack" would
now be a stale and wrong claim.

**What I could and did verify without a compiler**: brace/paren balance on every file I touched
(all balanced), zero live-code `dyn PluginApp` in the main SDK file (grep, comments excluded),
careful line-by-line reading of every signature I changed against its call sites within my owned
files, and the `#[dyn_enum]` rejection-condition checklist against `PluginApp`'s actual shape.
I did **not** get a real compile of `#[dyn_enum]` against a 51-method trait this large — the
dyn-enum-macro report's own largest test (`tests/scale.rs`) is a 45-method synthetic trait, close
but not identical; I was not able to reproduce that kind of standalone probe for `PluginApp`
itself within this packet's budget (its real signature pulls in dozens of domain types —
`ActionMeta`, `InvocationResult`, `HistoryPatch`, `MediaArtifact`, etc. — that would need
faithful stubs to probe honestly, and I judged that not worth the time against the actual
blocker being three crates upstream regardless).

---

## 5. What is NOT done — summary for the next packet

- `plugin_runtime`'s `GuestHost<PA>` restructuring (~56 fns / ~4,300 lines) — §3a. This is the
  actual remaining blocker for the crate to type-check at all, once `os-kernel` clears.
- `VcsArtifactApp`'s `SpaceMember`/`children` generic threading (store-dedyn's `open_child` lease,
  second half) — §3b, genuine design gap, needs a coordinator decision.
- Pervasive pre-existing missing-`.await` damage throughout the ~20,800-line main file and the
  964-line builder file, OUTSIDE the specific functions I directly rewrote for `PA` threading —
  confirmed present (`.encode_pack()`/`.decode_pack()` alone has ~10+ more un-awaited call sites
  I did not chase; `plugin_builder_dependency_tests`' 6 tests and `schema_stamping_tests`' 3 tests
  are type-correct for `PA` now but still have un-awaited builder chains). This is squarely
  `insert-await.py`'s job once a real `cargo check --message-format=json` is obtainable against
  this crate — blocked by the same upstream crate as everything else.
- Two other SDK traits reference `dyn` types outside `PluginApp`'s own family and were NOT in this
  packet's scope: `PayloadSource`/`ResourceResolver`/`PayloadSink` (seen in the os-kernel error
  dump above, live in `🚪️io/🦀️component.rs` — not my owned path).
- `#[dyn_enum]` was never actually compiled against the real 51-method `PluginApp` trait — see §4's
  closing paragraph.

## 6. Friction with `dyn_enum`/`dyn_enum_close!` at this scale (51 methods), for the ~50 more
   families that follow this recipe

- **Bare-fn-pointer factories compose cleanly with the macro, but require a SEPARATE manual
  refactor first.** `#[dyn_enum]` itself only cares about the TRAIT; the actual `Box<dyn Fn() ->
  Box<dyn PluginApp>>`-style capturing-closure factories that exist all over a codebase (this SDK
  had at least 5 of them: `SurfaceDeclaration.factory`, `editor_surface`/`viewer_surface`'s inner
  fns, `document_app`/`viewer`/`editor` on `PluginBuilder`) have to be converted to the
  "definition travels with a bare fn pointer, registry rebuilt inside the fn body" shape by hand,
  independently of the macro. Worth calling out explicitly in the recipe doc for the next ~50
  families, since it is easy to assume the macro alone solves object-safety end to end.
- **Type-inference fallout from removing `Box<dyn Trait>` is real and easy to miss.** Every call
  site that used to construct a trait object with NOTHING else pinning the concrete type down (a
  test that builds a `Plugin`/`PluginBuilder` and never actually creates an app) silently breaks
  once the field becomes `PA: PluginApp` generic — not a macro bug, but a consequence of the whole
  family this macro enables, worth flagging to every future `dyn_enum` adopter as a checklist item:
  grep for constructors of the now-generic type with no turbofish and no type-annotated binding.
  The `NoFoo {}` zero-variant default (already in the macro's own recipe §7) is the right answer
  for genuine "no instances needed" cases; a small local enum (like `SchemaStampApps` here) is the
  right answer when a test genuinely needs ≥1 concrete variant.
- **`resolve_ready` is used constantly as the E4/E1 bridge, and needs to be genuinely in scope
  wherever a fn-pointer thunk needs it** — inside a `#[macro_export]`ed `macro_rules!` that expands
  in a DIFFERENT crate (like `plugin_exports!`), it has to be spelled `$crate::app::resolve_ready`
  (or wherever it actually lives), not assumed to be glob-imported; got this wrong once during this
  packet (had to check the real re-export path rather than guessing `$crate::resolve_ready`).
- **Match guards cannot `.await`.** Not a `dyn_enum` finding specifically, but a hazard every
  method-conversion sweep like this hits: any `if <async call>` inside a `match` arm guard breaks
  and needs the value hoisted above the `match` first. Found and fixed one instance
  (`consume_media`'s `schema == self.document_schema()` guard).
- **`Iterator::fold`/`Once::call_once`/`OnceLock::get_or_init`/`LocalKey::with` closures cannot
  `.await` either** (all fixed-sync `FnMut`/`FnOnce` signatures from std) — this is the single
  largest source of "async fn called from a place that structurally cannot await it" across this
  whole codebase, confirmed pervasive both inside my owned file (fixed several instances) and in
  the upstream `os-kernel` error dump (§4). Worth its own line item in whatever briefs the next 50
  `dyn_enum` families, since it recurs far more often than the macro's own edge cases do.

---

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (extensive — trait, declaration
  tree, `Plugin`/`AppInstance`/`PluginProgram`, exchange helpers, E4 tags, S4 fixes, leases,
  `NoPluginApp`/`FixtureApps`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (extensive —
  `PluginBuilder<State, PA>`, `document_app`/`viewer`/`editor` factory conversion, `try_build`
  await fixes, `SchemaStampApps`, test call-site turbofish)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs` (E4 fixes +
  test awaits)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs` (E4 fixes +
  test awaits)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/📖️body/🦀️component.rs` (E5 fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (dispatch-macros
  dependency, async-test-attr dev-dependency)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📦️glue.rs`
  (`#![allow(async_fn_in_trait)]`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/Cargo.toml` +
  `📦️glue.rs` (async-test-attr)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/**` (4 files),
  `⚛️reactor/📸️checkpoint/🦀️component.rs`, `⚛️reactor/🩹️patches/🦀️component.rs` — `async-test-attr.py`
  mechanical `#[test]` → `#[async_test]` rewrite only, no hand edits.
- Ticket-folder scratch: `terra-sdkdedyn-scan1.txt`, `terra-sdkdedyn-apply1.txt`,
  `terra-sdkdedyn-asynctest-scan1.json` (all `.txt`/`.json`, none `.log`, per rule 5).

**Not touched, verified**: `🖥️host/**` (grepped my own diff — zero hits), `🏪️store/**`, `🛢️db/**`,
`🚪️io/**`, `🎒️pack/**`, `🧮️math/**`, `⏳️async/**`, `🔀️dispatch/**`, root `Cargo.toml` (diff
confirmed empty), anything under `✏️s/`.
