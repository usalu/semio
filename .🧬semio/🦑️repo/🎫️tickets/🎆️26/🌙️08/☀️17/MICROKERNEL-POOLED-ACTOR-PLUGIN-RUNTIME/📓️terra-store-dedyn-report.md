# 📓️ terra — store-dedyn report

Packet: `store-dedyn`, owned path `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**`.
File: `🏪️store/🦀️component.rs`.

## 0. Critical process finding — READ FIRST

**This file was overwritten by another concurrent session TWICE during this packet's execution**,
each time destroying already-completed, verified work (the entire Backbone family enum conversion,
the `NoBlobStore`/`MemberLinkResolver` genericization, and every `async`/`.await` I had added — the
file's `async fn` count dropped from ~739 to 2 between two of my own reads a few minutes apart, with
zero git commits in between). `🏪️store/**` is this packet's **exclusive** owned path per the brief;
another process editing it concurrently is a violation of the packet-ownership model, not something
I can fix myself. I redid the work twice against the file's then-current state. **sol: please
determine who/what is touching this file and stop it, or accept that this report's diffs may need a
third redo if it happens again post-report.**

Mid-session the coordinator also posted a new binding ruling (**R7**, added to `📌️important.md`)
requiring `async fn` in traits be kept (not reverted to sync) with `#![allow(async_fn_in_trait)]` at
crate root, and explicitly named `SpaceMember`/`Backbone`/`BackbonePort` as traits that must **not**
carry a `Send` bound. Both of my rebuilds target that ruling; the first (now-destroyed) pass had
predated it and used bare sync signatures.

## 1. Verified counts (python3 over the absolute path, per the brief's own warning about
shell/grep under-reporting on emoji paths)

At session start (before any edits): 12,383 lines, 739 `async fn`, 116 total `dyn <ident>`, broken
into: `dyn SpaceMember` **95**, `dyn Backbone` **7**, `dyn BackbonePort` **3**,
`dyn BackboneChannelPort` **2** (not named in the brief — found live, zero implementors anywhere in
the tree besides one old ticket's pre-patch scratch file), `dyn ChildStoreFactory` **4**,
`dyn BlobStore` **2** (also not named in the brief — a fourth family, see §3), `dyn std::any::Any`
**3** (std, R1-permitted, untouched). The brief's own header table undercounted `dyn Backbone`
because it silently folded `BackbonePort`/`BackboneChannelPort` into it (7+3+2=12).

At report time (current disk state, after both rebuilds): **zero** `dyn SpaceMember` /
`dyn Backbone` / `dyn BackbonePort` / `dyn BackboneChannelPort` / `dyn BlobStore` /
`dyn ChildStoreFactory` occurrences in actual code — every remaining hit (11 total, all `SpaceMember`
or `Backbone`) is inside a doc comment (`///`) or a `//` code comment, verified line-by-line.

## 2. Work item A — Backbone / BackbonePort / BackboneChannelPort — DONE

- `pub enum Backbones { Port(PortBackbone), Memory(MemoryBackbone), Channel(ChannelBackbone) }` +
  `impl Backbone for Backbones` (match-delegation, `async`).
- `pub enum BackbonePorts { Memory(MemoryBackbonePort), LocalStorage(LocalStorageBackbonePort) }` +
  `impl BackbonePort for BackbonePorts` (match-delegation) — the one family where the enum ALSO
  implements the trait, per the brief, for 🪐️space's blanket `impl<T: BackbonePort> ...`.
  `HOST_BACKBONE_PORT`/`set_host_backbone_port`/`host_backbone_port` now use `Arc<BackbonePorts>`.
- **New finding, not in the design doc**: `PortBackbone.channel: Option<Arc<dyn BackboneChannelPort>>`
  is a third dyn seam in this family. Zero live implementors exist (confirmed repo-wide; the only
  `impl store::BackboneChannelPort for HostBackboneChannel` is in an unrelated CLOSED ticket's
  `.pre-patch.rs` scratch file). Added `pub enum BackboneChannelPorts {}` (uninhabited, same shape as
  `NoMembers`) + `impl BackboneChannelPort for BackboneChannelPorts` (`match *self {}` bodies).
  `PortBackbone.channel`/`with_channel` now take `Arc<BackboneChannelPorts>` — behaviourally
  identical (still always `None` in practice) but fully dyn-free and ready for a real implementor to
  land as a new variant later.
- `ArtifactStore.backbone: Option<Backbones>` (was `Option<Box<dyn Backbone>>`); `attach_backbone`/
  `detach_backbone`/`attach_backbone_uri` take/return `Backbones` by value (no box, matching the
  brief's SDK-site guidance applied symmetrically on the store side too).
- Fixed real, pre-existing missing-`.await` bugs in `pump()`/`flush_outbound()`/`dispatch()`'s two
  internal calls to them/`tick()` — these are the direct Backbone-trait consumers, squarely in this
  family, not "elsewhere" fallout.
- Per **R7** (coordinator ruling, live mid-session): removed the `: Send + Sync` supertrait bound
  from `Backbone` and `BackbonePort` (Send now comes structurally from the concrete `Backbones`/
  `BackbonePorts` enum at each spawn site, never from the trait). `BackboneChannelPort` was not named
  by the coordinator and was left with `Send + Sync` (zero implementors either way, low risk).
- Added `#![allow(async_fn_in_trait)]` at the crate's top doc-comment block, with the R3/R7 rationale
  comment the coordinator specified.
- `resolve_backbone` (wasm-only) returns `Result<Backbones, VcsError>` directly.
- All in-file test call sites (`attach_backbone(Box::new(x))` → `attach_backbone(Backbones::Memory(x))`
  / `::Channel(x)`) updated mechanically (10 sites).

## 3. `BlobStore` — a fourth family, not named in the brief — DONE

`BlobStore` is defined in `🏪️store` but has real implementors in **other crates** (`🏃️run`'s
`FileBlobStore`/`InMemoryBlobStore`, `🪐️space`'s test-only `TestBlobStore`) plus an `impl` for
`FolderSqliteStorage` inside `🏪️store/🔄️sync/🦀️component.rs` (in-scope, no dyn there — just an
`impl`, not an object). The only 2 `dyn BlobStore` sites in my owned file are both in
`MemberLinkResolver<D>`'s `blobs` field/`with_blobs`. Fixed by genericizing:
`MemberLinkResolver<D, B = NoBlobStore>`, `pub enum NoBlobStore {}` (uninhabited stand-in, same
pattern as `NoMembers`/`BackboneChannelPorts`) implementing `BlobStore` with `match *self {}` bodies.
`new()` is `impl<D> MemberLinkResolver<D, NoBlobStore>`, `with_blobs<B: BlobStore>` is a separate impl
block. **Cross-packet finding, lifted here per rule 8**: `🏃️run/🦀️component.rs` and
`🪐️space/🦀️component.rs` still have live `Arc<dyn BlobStore>`/`&dyn BlobStore` usages (not fixed —
out of my owned path); whichever packet owns those crates will hit the same E0038 once `BlobStore`'s
methods are (already are, on disk) `async fn`.

## 4. Work item B — the SpaceMember cluster — the 95-site classification and the STOP-condition verdict

**Classification of the 95 (now re-verified: 90 after the second revert's own edits, then back to a
comparable count) `dyn SpaceMember` sites**, by shape:

| shape | count (approx.) | resolution |
|---|---:|---|
| `Box<dyn SpaceMember>` storage (`SpaceHost.members: HashMap<String, ...>`) | 1 | `HashMap<String, M>` — no `Box` needed, `M` is `Sized` |
| `Box<dyn SpaceMember>` params/returns (`register_member`, `unregister_member`, `ChildStoreFactory::create/open`) | ~10 | `M` by value |
| `&dyn` / `&mut dyn` params (`SpaceHost::member`/`member_mut`, `CompositionCoordinator`/`TransactionCoordinator` `parent`/`children`/`peers`) | ~15 | `&M` / `&mut M` |
| slices/arrays of `(&mut dyn SpaceMember, ChildDispatch)` (dispatch_group/dispatch_peer_group/compensate/undo_group/redo_group) | ~20 | `&mut [(&mut M, ChildDispatch)]` |
| doc-comment mentions (non-code) | ~10 | left as prose, still accurate |
| test-module coercions (`as &mut dyn SpaceMember`) + array type annotations | ~40 | removed; tests already construct ONE concrete type per test (verified by reading — no test mixes two different `(P, Mutation)` pairs in one composition), so plain generic inference over `M` covers every test unchanged |
| `as_any_mut(&mut self) -> &mut dyn std::any::Any` | (not counted, R1-legal, std, unchanged) | kept |

**Judgement call (the explicit STOP condition in the brief):** the generic parameter threads through
**5** distinct public types — `SpaceHost<M = NoMembers>`, `TransactionCoordinator`/
`CompositionCoordinator` (method-level `<M: SpaceMember>` / `<M: SpaceMember + MemberFactory>` generics,
the STRUCT itself stays non-generic since it only holds `CompositionGraph`), `GroupReceipt<M>`,
`MemberFactory` (new trait, replaces `ChildStoreFactory`), `MemberLinkResolver<D, B>` (separate,
unrelated `B` param for `BlobStore`). This is well under the ~10-type threshold — **I did not use the
fleet-wide aggregator-enum fallback; pervasive generics were sufficient.**

One real surprise the brief's plan (§1.6) did not fully anticipate: `CompositionCoordinator::
dispatch_group`'s genesis path called a **global, cross-crate, runtime-registered** `Arc<dyn
ChildStoreFactory>` registry (`register_child_store_factory`/`child_store_factory`, keyed by
`ArtifactKindId` string) to construct genesis children of arbitrary kind at runtime — structurally
the same "open, cross-crate, runtime-registered set" shape design-dedyn.md §2 says CANNOT be closed
into an enum (the 163-composer fn-pointer-table precedent). This looked at first like the
fleet-wide-enum trigger. Resolution: it doesn't need the fallback, because the registry's job splits
cleanly — the *lookup-by-kind* moves into the generated enum's own `MemberFactory::create` match
(closed per-plugin, not global), and the coordinator itself just gets an added `M: MemberFactory`
bound and calls `M::create(&spec.dialect.artifact_kind, &child_id, &spec.dialect, &spec.initial_pack)`
directly — no registry, no fallback enum, no dyn.

## 5. What changed (production code)

- **Deleted**: `ChildStoreFactory` trait, `CHILD_STORE_FACTORY_REGISTRY` (global `OnceLock<RwLock<...>>`),
  `register_child_store_factory`, `child_store_factory`, `ChildStoreFactoryRegistryError`,
  `TypedChildStoreFactory<P, Mutation>`, `register_typed_child_store_factory`.
- **Added**: `pub trait MemberFactory: Sized { async fn create(kind, id, dialect, initial_pack) -> Result<Self, VcsError>; async fn open(kind, envelope_pack) -> Result<Self, VcsError>; }`
  (replaces `ChildStoreFactory`); `pub fn create_member_store<P, Mutation>(...)` /
  `pub fn open_member_store<P, Mutation>(...)` (free generic helpers extracted from
  `TypedChildStoreFactory`'s old body — every `space_members!`-generated `MemberFactory` impl calls
  these per variant); `pub enum NoMembers {}` (uninhabited default, `impl SpaceMember for NoMembers`
  + `impl MemberFactory for NoMembers` returning a `ValidationFailed` error, since an uninhabited type
  has no `&self` to match on for `MemberFactory`'s associated-function shape); the
  `space_members! { pub enum $name { $variant($kind_str, $schema_str) => $Type },+ }` `#[macro_export]`
  decl-macro, colocated immediately after `trait SpaceMember`'s blanket impl, generating both
  `impl SpaceMember` (22-method match-delegation) and `impl MemberFactory` (kind-match `create`/`open`).
- **Genericized**: `SpaceHost<M = NoMembers>` (all methods generic or M-agnostic as appropriate);
  `GroupReceipt<M>`; `TransactionCoordinator`/`CompositionCoordinator`'s `dispatch_group`/
  `dispatch_peer_group`/`dispatch_relation_group`/`compensate` (method-generic `<M: SpaceMember +
  MemberFactory>`) and `undo_group`/`redo_group` (method-generic `<M: SpaceMember>`, weaker bound,
  correctly scoped since they never construct a member).
- `SpaceMember` trait: `: Send` supertrait removed (**R7**), every method `async fn`, blanket
  `impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>` likewise, with `.await` added on
  every internal cross-call (`self.dispatch(...)`, etc. — `dispatch()` itself had to become `async
  fn` too since it's the direct caller; its OWN callers throughout the crate are the "elsewhere"
  fallout described below, not touched).
- Test-module fixture: this crate's `#[cfg(test)] mod tests` already had (pre-existing, not mine) a
  shadow `struct ArtifactStore<P, Mutation>(super::ArtifactStore<P, Mutation>)` with `Deref`/
  `DerefMut` to the real store plus a hand-written `impl SpaceMember for` it — built originally for
  an `as_any_mut` downcast test. I asyncified that existing impl and added `impl MemberFactory for`
  it (schema fixed at `"demo/v1"`, `kind` argument ignored — these fixtures only ever register one
  kind per coordinator), which meant every `ArtifactStore::new(...)` construction already inside
  `mod tests` (dozens of call sites across the composition test suite) needed **no wrapper-type
  changes at all** — I only had to strip the now-redundant `as &mut dyn SpaceMember` casts (~35
  sites, mechanical `sed`-equivalent removal) and delete the separate `DemoChildFactory` fixture
  (its `ChildStoreFactory` impl no longer compiles; the shadow `ArtifactStore`'s own `MemberFactory`
  impl replaces it) plus its one call site (`register_child_store_factory` in the genesis-determinism
  test — deleted, no registry to register into any more). Two standalone `TypedChildStoreFactory`
  unit tests were rewritten to call `create_member_store`/`open_member_store` directly.

## 6. Acceptance — what I could and could not verify

**Static verification (done, trustworthy)**: python3 regex over the full file, re-run after every
edit batch, confirms **zero** `dyn SpaceMember` / `dyn Backbone` / `dyn BackbonePort` /
`dyn BackboneChannelPort` / `dyn BlobStore` / `dyn ChildStoreFactory` in actual code (comments only).

**`cargo check -p semio-framework-os-kernel --lib`**: run 4 times
(`CARGO_TARGET_DIR=.../scratchpad/target-store`, foreground, one turn each — logs
`terra-storededyn-check1.txt` through `check4.txt` in this ticket folder). **Every run failed before
reaching this crate at all**, each time in a different upstream dependency, each time from the SAME
root cause (universal-async codemod fallout — missing `.await`, or `async fn` where the language
forbids it) in a crate **outside my owned path**:
- run 1/2: `semio-framework-hash` (1 error) then `semio-framework-os-kernel-dsl-derive` (8 errors,
  `async fn` on proc-macro entry points — an **E3 violation**, someone else's packet).
- run 3/4: `semio-framework-replication` (130–209 errors: missing `.await` throughout
  `🌱️value/🔀️serde`, `⚠️diagnostic`'s `Fault::new`/`FaultCode::new` called sync inside `From` impls
  that can't be async themselves (E1), and one `E0733` recursive-async-fn-needs-Box::pin in
  `📡️wire`).

I have **never** obtained a `cargo check` that reaches `semio-framework-os-kernel`'s own diagnostics.
Per the acceptance section's own framing ("expect a large error count... the crate has its own
universal-async fallout beyond your families; that is NOT all yours to fix") and rule 21 ("a negative
result from a query that cannot report its own failure is not evidence of absence") I am reporting
this precisely rather than claiming a check that didn't happen: **my own family's correctness rests
on careful reading + the static dyn-census above, not a green (or even a same-crate-red) compiler
run.** Coordinator: this crate's build is currently unreachable for ANY packet downstream of
`semio-framework-hash`/`-dsl-derive`/`-replication` until those three are fixed — that's worth
knowing regardless of this packet.

## 7. Lease-requests (files outside `🏪️store/**` that now need a companion change)

```lease-request
Owner: whichever packet owns 🔌️plugin/📦️packages/🦀️rust (SDK, ATOMIC per R6 — sdk-dedyn?)
File: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs
Needed changes:
  1. attach_backbone(&mut self, backbone: Box<dyn store::Backbone>) at :9833/:12445/:15518
     → attach_backbone(&mut self, backbone: store::Backbones). The PortBackbone call site (:15518)
     becomes store::Backbones::Port(store::PortBackbone::new(uri)).
  2. open_child (around :10456) currently does:
       let factory = store::child_store_factory(&kind)...;
       factory.open(envelope_pack)
     store::child_store_factory / store::ChildStoreFactory / store::register_child_store_factory /
     store::register_typed_child_store_factory / store::TypedChildStoreFactory are ALL DELETED
     (O1 — the global dyn registry is exactly the seam being removed). Replace with:
     M::open(kind_str, envelope_pack) where M is the plugin's own space_members!-generated enum
     (store::MemberFactory trait, store::space_members! macro — both now exported from store).
     This requires VcsArtifactApp<A, M = store::NoMembers> to actually thread M through to whatever
     calls open_child today (design-dedyn.md §1.6 already specifies this second type param).
Why I can't do this myself: 🔌️plugin/🦀️component.rs is explicitly NOT my owned path (SDK, separate
ATOMIC packet per the brief). Both changes are mechanical once VcsArtifactApp's M param exists.
```

```lease-request
Owner: whichever packet owns ✏️s/🔌️plugins/🗄️stdio (fleet)
File: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs
Needed changes: `SemioChildStoreFactory` (struct at :85) currently `impl dsl::ChildStoreFactory for
SemioChildStoreFactory` (:122) and calls `dsl::register_child_store_factory(kind, Arc::new(
SemioChildStoreFactory))` (:170) at plugin init. Both `dsl::ChildStoreFactory` and
`dsl::register_child_store_factory` are DELETED. `SemioChildStoreFactory::create`/`open` already
dispatch via a macro-generated `match $name { ... TypedChildStoreFactory::<$snapshot, $mutation>
::new(STDIO_SEMIO_DOCUMENT_SCHEMA).create(id, dialect, initial_pack) ...}` over every registered
subset — this is ALREADY structurally a closed per-kind match, so converting it to
`dsl::space_members! { pub enum SemioMembers { ... } }` (with each `$name` as a variant, kind string
= the existing `stringify!($name)` match arm, schema = STDIO_SEMIO_DOCUMENT_SCHEMA) is mechanical,
not a redesign. `dsl::TypedChildStoreFactory` is deleted too; each generated arm's
`.create(id, dialect, initial_pack)` becomes `dsl::create_member_store(STDIO_SEMIO_DOCUMENT_SCHEMA,
id, dialect, initial_pack)` (now a free fn, not a factory method).
Why I can't do this myself: ✏️s/🔌️plugins/** is out of my owned path.
```

## 8. Missing-`.await` fallout — counted, not chased (per acceptance section instruction)

Outside my four/five families' own call chains, this crate's `#[test]` functions are ALSO broken for
a SEPARATE, pre-existing reason not mine to fix: `#[test] async fn ...` is not valid Rust without a
wrapper macro (design-dedyn.md §3 S3, `#[semio_framework_async_macros::async_test]`, a different
packet's job — `framework-tests`/`fleet-codemods` per R5). I did not add that macro or touch
`#[test]` attributes. Every test I touched keeps calling `ArtifactStore::dispatch(...)`/`.tick()`/
etc. synchronously (unchanged from before my edits) — those calls will need `.await` once the
`async_test` wrapper lands; that is squarely "elsewhere," the same S5 insert-await fixpoint loop
scope named in my brief.

## 9. Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (only file touched — everything
  above).
- Ticket-folder logs: `terra-storededyn-check1.txt` … `check4.txt` (cargo check attempts, all
  blocked upstream, see §6).

## 10. Summary for the coordinator

- Work item A (Backbone/BackbonePort/BackboneChannelPort) + the undocumented 4th family (BlobStore):
  **complete, dyn-free, async, R7-compliant.**
- Work item B (SpaceMember/ChildStoreFactory/SpaceHost/CompositionCoordinator): **complete** — every
  `dyn SpaceMember`/`dyn ChildStoreFactory` in owned-path code is gone, replaced with generics +
  `MemberFactory` + `space_members!` per the brief's design, without needing the fleet-wide-enum
  fallback.
- Not verified by a green (or same-crate) compiler run — blocked by out-of-scope upstream damage in
  `semio-framework-hash`, `semio-framework-os-kernel-dsl-derive`, `semio-framework-replication` across
  4 attempts.
- Two lease-requests filed (SDK `attach_backbone`/`open_child`, stdio/semio `ChildStoreFactory`
  migration).
- **This file was overwritten by a concurrent session twice mid-packet.** If it happens a third time,
  the diffs above are the reference to restore from — I no longer have session budget to redo again.
