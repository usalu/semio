# Full-Dialect Member Factory P0 Change Inventory

## Scope and evidence

Read-only current-source audit, 2026-09-04. No Rust or Nx command was run. This is the bounded P0 needed before a composed child can be selected or restored honestly. It does not materialize the Norm `q_k`/climate child values, mint parent child references, or make every Norm app a member host.

Current source is **RED**. The new neutral fixture below describes the intended contract, but the compiled Rust contract still dispatches an arbitrary string and admits a persisted envelope without checking its requested full coordinate.

| Boundary | Current evidence | Required P0 result |
|---|---|---|
| Factory API | [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17807) has `create(kind: &str, … dialect)` and `open(kind: &str, …)`. | Both operations receive the complete requested `ArtifactRef`/`ArtifactDialect`; there is no independently chosen string discriminator. |
| Closed binding | [`space_members!`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17994) accepts `($kind, $schema)` and matches only `$kind`. | Every arm declares exactly one full `(artifact_kind, standard, subset)` coordinate plus its snapshot schema, and matches all three fields exactly. |
| Restore authority | [`open_member_store`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2997) parses the envelope but checks only `owned ⇒ dialect present`. | It checks parsed persisted schema, full dialect, id, and child ownership against the expected child before constructing or publishing a store. |
| Production bypass | [`impl MemberFactory for ArtifactStore`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3021) uses caller `kind` as schema in `create`, and ignores it in `open`. | Delete this production blanket implementation. Production must use an explicit closed `space_members!` enum. |
| Generic VCS caller | [`VcsArtifactApp::open_child`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19877) passes only `dialect.artifact_kind` to `M::open`. | Pass the exact expected child ref (id plus all three dialect fields) and do not admit/insert/publish until restore returns success. |
| Genesis caller | [`CompositionCoordinator::dispatch_group`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19168) passes only `spec.dialect.artifact_kind` to `Mc::create`. | Pass `child_id` and the complete `spec.dialect`; a macro arm validates it before decoding `initial_pack` or constructing a child. |

## Exact P0 contract

The factory needs two distinct inputs, neither derived from the other:

```rust
async fn create(id: &str, dialect: &ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError>;
async fn open(expected: &ArtifactRef, envelope_pack: &[u8]) -> Result<Self, VcsError>;
```

`create` has no persisted envelope, so `(id, dialect)` is its authority. `open` must receive the entire `ArtifactRef`: a dialect alone cannot reject an envelope substituted under a different child id.

For every macro arm, require four static literals: `artifact_kind`, `standard`, `subset`, and body `schema`. The triple is the closed selection authority; schema is a separate body-codec authority. A schema such as `stdio.semio` cannot infer which of the 18 `semio` subsets is intended. The macro must check the requested triple before its arm reaches typed decode/open.

The resulting admission order is:

1. Parse the caller's full requested coordinate before selecting a macro arm; reject any unlisted triple.
2. On create, reject an empty initial pack, decode only after the arm matches, create with that arm's static schema, and stamp the requested dialect.
3. On open, decode the length-framed pack plus `.spr` history composition. The persisted `envelope.id`, `envelope.schema`, `envelope.dialect`, and owned-child state must equal the expected ref and selected arm before `ArtifactStore::new`.
4. Only a successful factory result reaches `open_child`'s child-map admission, composition graph insertion, or content publication.

The persisted authority is the `REC_COMPOSITION` history overlay: [`history_composition_from_envelope`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10737) serializes it and [`apply_history_composition`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10762) restores `envelope.dialect`. [`parse_document_pack`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11201) is the only suitable pre-construction parse path. The length framing helper alone is not authority.

P0 should also reject a non-owned envelope for a member restore. Parent-to-slot reference membership and composed-value materialization are P1, but an arbitrary root envelope must not be accepted as a child merely because it has a matching dialect.

## Exact source changes

1. In [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs), change `MemberFactory`, `NoMembers`, macro documentation, and generated macro implementations at lines 17807–18131 to the full-coordinate contract. Make the macro grammar require the coordinate triple for every arm. Do not retain a string-key overload.
2. Change `create_member_store` (line 2974) to take static selected schema plus the full dialect. It already rejects an empty pack; retain that law.
3. Change `open_member_store` (line 2997) to receive static selected schema plus `expected: &ArtifactRef`; after `parse_document_pack`, reject before `ArtifactStore::new` when id, schema, full dialect, or member ownership disagrees. The planned source predicate in [`composition/member-dialect/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts:37) correctly captures schema/dialect but must also cover id and ownership.
4. Delete the production blanket implementation at lines 3021–3033. It turns caller input into a schema and makes `open` ignore its discriminator. There is no production caller that needs this implementation.
5. Change the coordinator genesis call at line 19168 to `Mc::create(&child_id, &spec.dialect, …)`.
6. Change `VcsArtifactApp::open_child` at lines 19877–19910 to assemble `ArtifactRef { artifact_id: child_id, dialect }` and pass it to `M::open`. Retain its existing cancel-admission error path; do not reserve or publish before factory validation.
7. Convert stdio [`SemioMembers`](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1341) all 18 arms to full fixed coordinates. Remove `create_semio_member`'s `dialect.subset` dispatch, `open_semio_member`'s persisted-subset recovery, and `subset_of_persisted_envelope` (lines 1366–1386). The generic VCS path, not a helper that learns its selection from untrusted persisted bytes, is the P0 route.

## Census: production versus fixture conversions

There is no production use of a bare `ArtifactStore<P, Mutation>` as a VCS member type (`Mc`) or `VcsArtifactApp` child member. The only production macro invocation is `SemioMembers`; its 18 arms are the intended closed binding.

The only other macro invocations are fixtures:

| Location | Current form | P0 conversion |
|---|---|---|
| [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21695) | `RetainedTestMembers`, two arms | Supply each full demo coordinate. |
| [`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34624) | one-arm `TestMembers` | Supply its full coordinate; extend its full persist/reload test to mismatched id and all three mismatched coordinate components. |
| [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21363) | `#[cfg(test)]` blanket `MemberFactory for ArtifactStore` with arbitrary kind and default empty genesis | Remove it. Define a small explicit `DemoMembers` fixture through the same macro and wrap only member children. This preserves a plain parent store and avoids inventing `MemberStoreOwner` metadata. |

The direct bare-`ArtifactStore` `Mc` usages are all test-only in [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26241): atomicity, ownership/policy, genesis-determinism, and receipt tests. Convert their child arrays to the explicit test enum; retain the parent as `ArtifactStore`. The `compensate` tests at lines 26308 and 26348 use `SpaceMember` directly and need no factory conversion. Replace the wrapper test at lines 26061–26076, which currently invokes bare `ArtifactStore` factory methods, with a closed-enum test. This is a test conversion, not a production compatibility layer.

## Neutral fixture and registered gate

The in-flight language-neutral fixture already exists at [`composition/member-dialect/🧪️tests/🔣️.json`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json) with two same-kind/different-subset bindings and 13 create/open rows. Its AJV oracle at [`📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts:6) is valuable but **not executable evidence yet**: it reads for a new full-coordinate Rust contract that current source does not contain.

Extend it with at least:

- persisted id mismatch;
- non-owned persisted envelope;
- every wrong triple component under both create and open;
- an unregistered but otherwise well-formed request;
- no post-failure child-map or publication observation in the generic VCS law.

The source-only prerequisite is already registered as `bun nx run @semio-tech/framework-os-kernel:test-member-dialect-source --skip-nx-cache` ([`project.json`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json:36), [`📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:58)). It runs the Bun/AJV fixture only and is insufficient for acceptance. Add a separate registered executable `member-dialect-check` at that owner. It must first run the neutral fixture, then use Cargo `--list` to require exactly one fully-qualified Rust test for each named law before `--exact` execution; it must not use substring selection or skips. The Rust laws should exercise real pack/SPR parsing, macro arm selection, VCS `open_child` no-publication failure, and coordinator genesis rejection. The stdio project should add a separate all-18-arm full-coordinate law using real `SemioMembers` rather than reintroducing a subset helper.

Add any launch item only in [`.vscode/🧩️launch.seed.jsonc`](../../../../../../.vscode/🧩️launch.seed.jsonc), then run `bun nx run @semio-tech/plugin-registry:generate` followed by `bun nx run @semio-tech/plugin-registry:check-generated`. Never edit generated `.vscode/launch.json` directly.

## P0 acceptance and nonclaims

P0 accepts only a closed full-coordinate factory with exact pre-construction persisted-envelope validation and no factory bypass. It makes neither a claim that Norm composed values survive reload nor that all apps use member factories. Those require the later child-reference/genesis and member-host packets.

## Live landing review: still RED

The tree changed during this audit. At this read, the production blanket implementation has been removed and the macro now requires four literals and matches `(artifact_kind, standard, subset)` ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17782), [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17968)). Genesis now passes `id` plus full dialect (line 19142). These are source-closed pieces only.

Three material gaps remain in those live bytes:

1. `MemberFactory::open` at line 17784 still receives only a dialect. `open_child` at [`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19884) therefore cannot cause the helper to compare the recovered `envelope.id` with its `child_id`. A same-dialect envelope for another child may be inserted under the caller's id. Change `open` to take `&ArtifactRef`, and check the recovered id before construction.
2. `open_member_store` at [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2997) now checks schema and dialect, but not `envelope.owner`. A root envelope with an otherwise matching dialect still crosses the member restore boundary. Require an owned child and `owner.child_id == expected.artifact_id`; parent/slot edge verification remains P1.
3. Stdio still uses the former two-literal macro arms and subset dispatch/recovery ([`semio/🦀️.rs`](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1341)). It is incompatible with the new macro grammar and continues to recover its selection from untrusted persisted bytes. Convert all 18 arms and delete the subset helper as a single atomic source change before any compile claim.

The test-only generic `MemberFactory for ArtifactStore` remains at `store/🦀️.rs:21337`; it has begun checking the one demo coordinate but is still a blanket implementation. Replace it with the explicit closed fixture enum described above so the test corpus does not preserve an alternate factory shape.

## Parent-coordinate constructor seam

The only sound source of the parent coordinate is the app's declared dialect, not `DOCUMENT_SCHEMA` and not a value recovered from a child. The runtime currently loses that source at [`VcsArtifactApp::with_registry_on_bus`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19114): it creates the parent envelope from `A::DOCUMENT_SCHEMA` without setting `envelope.dialect`. The two group paths then invent `DOCUMENT_SCHEMA@native/*` if it remains absent ([`dispatch_emit_group`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20750) and [`dispatch_group_history_action`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20954)). That synthetic coordinate cannot satisfy exact owner comparison.

`Dialect` is the correct compile-time authority: it is `Copy`, contains static kind/standard/subset literals, and converts once to owned `ArtifactDialect` ([`io/schema/🦀️.rs`](../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:48), line 74). `ArtifactEditor` and `ArtifactViewer` already require `const DIALECT: Dialect`; their adapters can forward it without parsing a surface id ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25911), [`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26474), [`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26738)).

Make `const DIALECT: Dialect` required on runtime `ArtifactApp`, then forward `E::DIALECT` and `V::DIALECT` in `EditorApp`/`ViewerApp`. This is intentionally not an optional method: a missing coordinate must fail compilation, rather than reintroduce a fallback. In `with_registry_on_bus`, convert `A::DIALECT` once and stamp it into the parent envelope before `ArtifactStore::new`. Replace both synthetic fallbacks with a fail-closed exact-envelope read (or, stronger, compare `self.store.envelope().dialect` to `ArtifactDialect::from(A::DIALECT)` and fault on mismatch).

The ingress twin is [`ArtifactStoreReplacementAdmissionTarget::try_adopt_completed`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18484): before it asks the app to construct a replacement, reject/return the completed envelope unless both `envelope.schema == A::DOCUMENT_SCHEMA` and `envelope.dialect == Some(ArtifactDialect::from(A::DIALECT))`. Otherwise a restored parent may later supply an untrusted owner coordinate even though fresh construction was fixed.

The new `open_child` code should construct its `Some(OwnerRef)` from this exact parent reference plus its local `slot` and `child_id`, and pass the expected child `ArtifactRef` to `M::open`. The generic restore helper then applies the following exact rule before `ArtifactStore::new`:

```text
persisted.id == expected.artifact_id
&& persisted.schema == static_arm_schema
&& persisted.dialect == expected.dialect
&& persisted.owner == supplied_expected_owner
```

For an explicitly standalone/peer path, `supplied_expected_owner == None` means the persisted owner must be `None`; it is not permission to accept either kind of owner. This preserves a truthful standalone construction mode without admitting an arbitrary root or child envelope through the other mode.

The required direct `ArtifactApp` inventory is small: runtime adapters `EditorApp` and `ViewerApp`, internal fixtures `TestApp`, `KeyedTestApp`, and `CopyDrawApp`, fixture modules `DummyApp` and `TxnApp`, and production direct apps `ModuleApp` ([`playbook procedural`](../../../../../../✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:702)) and `SpaceApp` ([`space engine`](../../../../../../✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:583)). There are no other direct production `ArtifactApp` implementations. `ModuleApp` and `SpaceApp` already declare canonical surface ids (`s.playbook.procedural@1/*#editor` and `s.space.studio@1/*#editor`); give each an adjacent `Dialect` constant and retain a law that its surface id is derived from that coordinate. Fixtures must declare a real test coordinate too; do not infer it from their current free-form controller names.

The P0 Rust gate needs two extra exact laws: (1) fresh VCS construction stamps the author-declared parent dialect and a child restore receives that exact parent owner; (2) an ingress parent envelope with correct body schema but missing or different dialect is rejected before its initializer and does not replace/publish. The existing neutral fixture should add owner `none`/matching/mismatched-parent-slot-child rows. Neither law requires a native runtime claim.

### Live parent-seam re-read

The current landing uses `ArtifactApp::document_dialect() -> Option<ArtifactDialect>` (default `None`) and correctly forwards `Some(E::DIALECT.into())`/`Some(V::DIALECT.into())` from the editor/viewer adapters. `with_registry_on_bus` now stamps that option into its fresh envelope. The member half is source-closed: `MemberFactory::open` receives `expected` plus owner, `open_member_store` compares all four persisted facts before construction, and `open_child` constructs its parent/slot/id owner before reserve/admit.

It remains **RED** at this read because the two group routes still synthesize `DOCUMENT_SCHEMA@native/*` when the optional declaration is absent ([`dispatch_emit_group`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20758), [`dispatch_group_history_action`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20962)), and replacement ingress still calls its app initializer without schema/dialect admission ([`ArtifactStoreReplacementAdmissionTarget`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18484)). Either make runtime `ArtifactApp::DIALECT` mandatory as recommended above, or retain the optional seam but make every group/ingress path fail closed when it is absent/mismatched. No runtime gate has run.

## Current P0 child-admission atomicity audit — RED

This section supersedes the preceding optional-dialect observation: current `ArtifactApp` now has required `const DIALECT: Dialect` ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10985)), fresh construction stamps it ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19117)), and `open_member_store` rejects differing id, static schema, full dialect, or full owner before `ArtifactStore::new` ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2997)). No Rust runtime gate was run for this audit.

The factory's direct failure disposition is sound: `open_child` reserves a `ChildMemberAdmission`, invokes `M::open`, and cancels that exact reservation on `Err` before a map, graph, or root publication mutation ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19890)). The following post-factory paths remain materially non-atomic.

1. `open_child` publishes the member into `ChildMemberRegistry` before it calls `CompositionGraph::insert_owns` ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19908)). A conflicting graph edge returns an error while retaining a member that the fail-closed group dispatcher cannot consistently authorize.
2. The graph's advertised "same parent/slot" idempotence is not implemented. `insert_owns` rejects only a different parent, then overwrites the recorded slot for the same child id ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18651)). The registry key is `(slot, child_id)`, so two child-map entries with the same child id and different slots can be admitted while the graph silently changes the one authoritative slot. This is an ownership integrity defect, not merely error cleanup.
3. Reordering only graph before map does not close the transaction. `publish_child_content_member` remains fallible after map and graph mutation: it captures an erased snapshot through `member.snapshot_read_erased()` and builds a bounded immutable root ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19810), [`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8606)). A failed snapshot capture/capacity check therefore still leaves live map/graph state. The captured snapshot cannot simply be dropped: `ChildContentRetirement` returns it through the exact live member disposer ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8716)).
4. A queued checkout pin is removed before `checkout`, and that `Result` is deliberately discarded ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19901)). Any checkout error permanently loses the retry authority and violates the documented deferred-pin guarantee. The analogous live-child cascade also discards checkout errors at line 20031; that is a related source defect, but the lazy-open loss is P0-critical.
5. `register_child` has the same post-insertion publication escape: it inserts the member after a graph operation and then returns a `member: None` error when publication fails ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19950)). Any helper used for restore must serve registered/genesis adoption too, otherwise a constructed member bypasses the repaired transaction.
6. The genesis-receipt sibling `absorb_created_children` defaults a missing graph slot to `""`, inserts into the map, then performs the same fallible publication ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20707)). A corrupted/missing phase-two ownership edge is therefore not fail-closed. It must require an exact pre-existing graph `(parent, slot, child)` witness and use the same admission/commit helper.

### Required transaction shape

Use one private child-admission owner shared by `open_child` and `register_child`, not two independently ordered paths. It must retain the registry reservation, uninserted `M`, optional queued pin, graph insertion witness, and any prepared snapshot/publication owner until commit.

- Make graph admission exact: reject an existing `(parent, slot)` mismatch; return an `Inserted` versus `AlreadyExact` witness. A rollback must remove only an edge inserted by this turn and must match parent, slot, and child — `remove_owns(child_id)` alone is unsafe because it may erase a pre-existing exact edge.
- Move every ordinary rejection before publication: full child key, parent-envelope dialect, registry reservation, graph exactness/cycle preflight, root generation/retirement capacity, factory open, and queued checkout. Propagate checkout failure. Leave the pin in `pending_child_pins` until success; do not delete it merely to obtain a temporary value.
- Prepare the child root under an explicit cancellable snapshot-read owner, or make the final map/graph/root commit infallible after a successful preflight. If preparation performs `snapshot_read_erased`, its abort path must hand the exact read back through the uninserted member's disposer; raw drop is not valid. A graph/mapping mutation may occur only when that owner has a complete abort path. The successful `M::open` result itself is an owned resource: real `ArtifactStore` asserts terminal-empty in `Drop` ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16526)). Thus `open_child` cannot simply drop it when a later stage fails; its transaction abort needs bounded `SpaceMember::close_owned_step` retirement. `register_child` may return `M` only before any member-owned read/retirement is taken.
- Commit exactly once: install map, install graph edge if `Inserted`, replace root/retirement, advance generation, then consume the queued pin. Any error before this point returns the registration's `M` where applicable, cancels the exact reservation, retains the pin, and leaves map, graph, root, retirement registry, and generation unchanged.

### Required executable laws

Add exact real-member laws to the registered member-dialect check; the current Bun/AJV fixture is only source/oracle evidence.

- factory error after reservation: no child map entry, graph edge, root/generation change, retirement, or pin consumption;
- graph conflict and same-parent/different-slot conflict after a successful `M::open`: same no-publication result;
- checkout failure: reservation cancellation/no publication and the exact queued pin remains retryable;
- root snapshot/publication failure: the same no-publication result and exact snapshot disposer ownership;
- successful open and direct registration: exactly one map entry, exact graph `(parent,slot,child)`, one root generation, and one consumed pin;
- no duplicate child id across slots and no blind graph rollback of a pre-existing exact edge.
- receipt absorption with a missing graph witness: reject before map/root publication, never substitute the empty slot.

The neutral fixture should add id/slot conflict and failure-stage observations, but it cannot prove Rust snapshot disposal or transactional restoration by itself. This remains source RED until the shared admission implementation and non-vacuous exact Rust laws execute.

### Bounded shared admission design

The existing owners are sufficient, but their live/close split matters:

- `ChildMemberRetirement<M>` already performs the exact bounded `M::close_owned_step` loop ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8794)). `close_child_member` is only one `Option` consumed by the shutdown path, not live maintenance ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23375)); it cannot retain a failed live admission.
- `ChildContentRetirement` is the only correct route for an immutable root's erased snapshot: it returns the snapshot through the exact member's `retire_snapshot_read_erased` and verifies terminal emptiness ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8656)). The current `child_content_retirements` fixed registry is already live-pumped at maintenance stage 4 ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23773)).
- `ArtifactFixedRegistry<T>` has exactly 64 pre-admitted slots, does not allocate after construction, and faults if its authority cannot accept an owner ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15456)). It is the appropriate bounded container; a `Vec`/unbounded deferred-drop list is not.

Add a separate **fixed** `child_admission_abort_retirements: ArtifactFixedRegistry<ChildAdmissionRetirement<M>>`, an exact monotonic/ticket cursor, and one live maintenance cursor/stage. `ChildAdmissionRetirement` is not a second cleanup protocol: it composes the existing `ChildContentRetirement` snapshot-return phase with the existing `ChildMemberRetirement` close phase. Include this registry in `maintenance_step`, `close_step`, and `close_terminal_is_empty`. Before `M::open` or receipt consumption, pre-admit an abort ticket; exhaustion fails before the caller constructs/transfers an unreturnable member. Each maintenance turn advances one owner under its normal item/byte grant, removes it only after terminal-empty, and otherwise returns `Pending`/`Blocked` without spin.

Make the publication path two-phase so an admitted snapshot has no subsequent fallible root operation:

```text
ChildContentView::admit_member(slot, child) -> ChildContentAdmission
ChildContentView::capture_admitted(admission, dialect, &member)
    -> Result<PreparedChildRoot, Fault>
PreparedChildRoot::commit_into(app, graph_admission, child_member_admission) -> ()
```

`admit_member` performs the hash/probe/capacity decision before a snapshot lease exists. `capture_admitted` obtains `snapshot_read_erased`; after that success it uses the reserved location, so it cannot re-run a fallible root lookup. `commit_into` is deliberately infallible: registry insert, exact graph insert, root replacement/old-root `ChildContentRetirement` insertion, generation advance, and pin removal are assertions against the prior admission tickets. If capture fails before a lease is minted, direct registration returns its still-owned `M`; factory/receipt callers transfer it to the fixed abort registry. If a multi-child batch has already captured one or more snapshots when a later candidate fails, each captured candidate transfers to `ChildAdmissionRetirement`, which first retires that exact snapshot through its detached member and only then runs `ChildMemberRetirement`.

`CompositionGraph` needs an explicit `OwnsAdmission::{Insert, AlreadyExact}` ticket. Its preflight rejects a different parent **or slot** for an existing child and checks the cycle before any insertion. Its commit/remove APIs must require that ticket's exact `(parent, slot, child)` identity. A plain `remove_owns(child)` cannot be used by an abort path.

`SpaceMember` currently exposes only `document_id` ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17293)); consequently `register_child` cannot prove that a constructed store's dialect or owner agrees with its caller arguments. Add object-safe cloned `artifact_ref()` and `owner_ref()` reads to `SpaceMember`, with `ArtifactStore` implementation and `space_members!` delegation. Then:

- `open_child` requires the factory-returned member to equal its expected ref and owner;
- `register_child` requires exact id/dialect and either `None` (fresh member) or the exact expected owner. Only after all fallible preparation may it stamp `None` to that expected owner, then commit;
- `absorb_created_children` requires both an exact pre-existing graph witness and the receipt member's exact ref/owner. It must never use `slot_of(...).unwrap_or_default()`.

`absorb_created_children` must preflight the whole receipt before its first map/root commit. Bound it to the 64-slot publication authority, pre-admit every registry/graph/root/abort ticket, prepare all candidates, then make the commits. It currently performs per-element partial publication ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20707)); a later failure otherwise leaves a fraction of a group receipt visible. `CompositionCoordinator::dispatch_relation_group` has the same owner-liveness bug one stage earlier: its comment says earlier successful `Mc::create` values can simply drop on a later genesis error ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19134)), which contradicts `ArtifactStore::Drop`'s terminal assertion. Bound genesis to the same fixed batch limit and return/retain each created member through the same admission-abort owner on every later factory/graph failure.

#### Exact no-publication laws

The registered Rust gate needs these production-machine laws in addition to neutral fixture vectors:

1. **Open/factory failure:** factory error cancels its exact map admission; no graph/root/generation/pin/abort owner changes.
2. **Open/checkout failure:** root and graph stay unchanged, pin stays queued, one fixed abort ticket owns the opened member; repeated maintenance drains it under its grant with no leak or second close.
3. **Direct registration mismatch:** wrong id, full dialect, foreign owner, and same child/different slot all fail before owner stamping/map/graph/root; the caller receives its exact member unchanged.
4. **Snapshot capture failure:** direct registration returns its exact member; factory/receipt routes retain it once for bounded close; no map/graph/root publication occurs.
5. **Receipt batch failure at every ordinal:** zero receipt children become visible; all already-created/captured members have one bounded abort owner, which drains; no empty-slot fallback.
6. **Success:** exactly one map row, exact graph edge, root generation, and deferred pin consumption; old root retirement remains in the existing registry and drains through the exact current member.

No compile or runtime result is implied by this design audit.

## Live delegate, graph-law, and neutral-corpus inventory

## Live member-open rejection ownership review

Coordinator-observed session 48129 reached the first native member-admission assertion after
the four schema laws passed and the kernel unit binary compiled.  It then panicked in
`ArtifactEnvelope::drop`; this audit did not run that session.  Current source explains the
failure: `open_member_store` decodes the container and calls
`parse_document_pack` before it compares the resulting terminal envelope with the requested
identity ([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3114)).
The mismatch branch at line 3120 returns `Err` while still owning `ArtifactEnvelope`, whose
drop contract at lines 2390–2468 deliberately aborts unless a bounded app-owned retirement
authority detached every nested owner.  The app's
`child_admission_abort_retirements` cannot repair that branch: it only receives an already
constructed `M` after `M::open` returns ([`plugin/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19957)).

### Smallest sound P0 repair: header metadata preflight

The proposed repair is sound for this exact identity-rejection path:

1. Decode the outer document container, then decode its `.spr` once with the existing
   `DecodeOptions::default()`.
2. Before `P::decode_pack`, operation decode/replay, or `ArtifactEnvelope` construction,
   compare the decoded `HistoryLog.doc_id` and `HistoryLog.schema` with the requested
   `ArtifactRef` and the macro arm's static schema
   ([`history/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:22),
   [`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3109)).
3. Require a composition overlay with the exact requested dialect.  For an owned open, parse
   its parent URI with `ArtifactRef::parse_uri`, construct the equivalent `OwnerRef`, and
   require equality of parent, slot, and child id with the supplied expected owner.  Reject
   `owner.child_id != expected.artifact_id` before decoding history.  For a standalone open,
   require an absent persisted owner; `None` is not a wildcard.
4. Extract a private `parse_decoded_document_spr(pack, history_log)` from
   `parse_document_spr` and pass it the same decoded log.  This avoids a second decode and,
   critically, makes the preflight's last-wins composition semantics identical to the eventual
   parser's semantics ([`history/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1548-1622)).

Do not raw-scan `REC_COMPOSITION` or compare the parent URI text directly.  The current
decoder makes that extension last-wins, and `ArtifactRef::parse_uri` is the canonical
semantic comparison boundary ([`history/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1587-1591),
[`io schema/🦀️.rs`](../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:175-193)).
The static expected full coordinate is already canonical; equality therefore also rejects every
malformed or substituted overlay tuple.

This P0 repair is deliberately narrower than a general retained decoder.  It prevents the
observed wrong-id/schema/dialect/owner branch from ever constructing a terminal envelope.  It
does **not** make malformed history after the header cancellable, nor does it make every later
`ArtifactStore::new(envelope).await?` error safe: `ArtifactStore::new` still validates and
seeds runtime state after receiving the terminal envelope
([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13852-13882)).
Those paths require a separately retained initialization/envelope rejection authority, not a
blind close loop or `Drop`/ `forget` workaround.  The existing
`ArtifactStoreEnvelopeRetirement` already encodes the required bounded nested retirement but
needs the exact domain factories and a caller-owned retention slot
([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1250-1465),
[`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1971-2017)).

### Required narrow laws

The corrected first member law must observe all of the following with a real pack/`.spr`
fixture:

- wrong document id, static schema, each dialect component, missing composition, foreign parent,
  wrong slot, wrong child id, and a caller owner whose child id contradicts the requested ref all
  return a normal error before snapshot decode/replay, `ArtifactStore::new`, child map, graph,
  root, or abort-registry mutation;
- the successful row uses exactly one decoded `HistoryLog`, then reaches the existing typed
  restore and normal app close;
- a malformed body after an otherwise matching header is reported as a parse error without
  pretending header preflight supplied retained/cancellation ownership; and
- a later store-initialization rejection has its own retained-owner law before it is claimed
  safe.  It cannot be absorbed into the header-preflight acceptance claim.

The independent JSON/AJV fixture can cover the identity matrix but cannot observe typed
snapshot decode/replay suppression or terminal-envelope disposal.  The native exact law must
carry those observations.

This is a source-only re-read after the P0 factory authority landing.  It records the exact places
that a new read-only member identity/owner surface, exact graph admission, and bounded abort owner
must reach.  It does not claim that the pending child transaction compiles or executes.

### `SpaceMember` addition has four concrete implementation shapes

The proposed object-safe `artifact_ref() -> ArtifactRef` and `owner_ref() -> Option<OwnerRef>`
must be cloned from the authoritative envelope; neither may be reconstructed from the caller's
map key or current graph.  The complete current implementation/delegation inventory is:

| Shape | Exact site | Required change |
|---|---|---|
| Real store | [`SpaceMember for ArtifactStore`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17451) | Return `ArtifactRef { artifact_id: envelope.id.clone(), dialect: envelope.dialect.clone().expect(...) }` only if the envelope dialect is the invariant-bearing concrete store fact; return `envelope.owner.clone()` separately. Do not mint an identity from `P` or schema. |
| Empty set | [`SpaceMember for NoMembers`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17792) | Add the two unreachable `match *self {}` arms, so a missing arm remains a compile error rather than a default identity. |
| Every closed production/fixture enum | [`space_members!`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17974) | Add match delegation once beside the existing `document_id`, snapshot-retirement, and `set_owner` delegates. This reaches production [`SemioMembers`](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1333), test [`RetainedTestMembers`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21673), and plugin test [`TestMembers`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34644). There is no second production macro. |
| Store test wrapper | [`tests::ArtifactStore` delegation](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21205) | Delegate both reads to `.0`. This private wrapper is a separate nominal type, so the blanket real-store implementation does not cover it. |

`set_owner` alone cannot serve as the proof: it is a mutation and permits `register_child` to
stamp whatever the caller says before learning whether the constructed member was actually the
right document.  The new reads must be checked before any owner mutation.  The macro makes this a
closed, compiler-enforced delegate census rather than a per-plugin migration.

### Graph callers and current law gap

Only five non-test paths reach `CompositionGraph::insert_owns`:

1. [`CompositionGraph::sync_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18719) removes every existing outgoing edge of the parent, then calls `insert_owns` for each projected child. It is an additional atomicity boundary: a later collision/cycle currently leaves the old projection erased and an early fraction installed. Preflight an isolated candidate map first, then replace the outgoing edge set once; do not repair this with broad `remove_owns` calls.
2. [`TransactionCoordinator::dispatch_relation_group`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19145) installs genesis edges after `Mc::create` and `set_owner`. Its existing comment that earlier created members “simply get dropped” is false for a real `ArtifactStore`, whose `Drop` asserts terminal-empty. It needs the same pre-admitted bounded abort owner if a later factory/graph step fails.
3. [`VcsArtifactApp::open_child`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19910) and 4. [`register_child`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19951) are the two pending shared-transaction consumers.
5. `absorb_created_children` does not insert an edge itself, but depends on the graph at [`plugin/🦀️.rs:20710`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20710). It must require an exact `(parent, slot, child)` witness, not `slot_of(...).unwrap_or_default()`.

The sole direct graph law, [`composition_graph_owns_forest_rejects_second_owner_and_cycle`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26226), proves initial insertion, repeat-same-slot idempotence, a different-parent rejection, and a cycle. It does **not** prove same-parent/different-slot rejection, no mutation on a rejection, exact-ticket rollback, `sync_member` all-or-nothing replacement, or genesis owner retirement. The various group laws only seed one valid edge; they do not cover those admission states.

The minimal graph API is therefore a non-clonable `OwnsAdmission` returned by a pure preflight:

```text
preflight_owns(parent, slot, child) -> Insert(ticket) | AlreadyExact(ticket)
commit_owns(ticket) -> ()
cancel_insert(ticket) -> ()
```

`preflight_owns` rejects a different parent **and a different slot** and performs the cycle test
against the same candidate state. `cancel_insert` is valid only for `Insert` and checks all three
identity fields. `AlreadyExact` never removes an older edge. For `sync_member`, build and validate
the candidate outgoing set in a local map, then atomically install it; an individual edge ticket
is insufficient when a later child candidate fails.

### Language-neutral atomicity corpus

Add a separate schema-first corpus, for example
`store/🧩️composition/🪪️child-admission/🧪️tests/🔣️.json`, rather than overloading the
existing full-dialect fixture. Its Bun/AJV oracle may validate input/output states and bounds, but
the Rust exact laws below must prove real snapshot/member disposal. Use fixed string IDs, literal
coordinates, and stage names; no implementation-language error text.

| Family | Required neutral rows | Required observable result |
|---|---|---|
| `graph.insert` | empty insert; exact repeat; different parent; same parent/different slot; self and indirect cycle | `Inserted`/`AlreadyExact` only for the first two; every rejection preserves the original exact edge and leaves no new edge. |
| `graph.sync` | replace valid old child set; foreign-owner collision at ordinal 0 and N; duplicate child under two slots; cycle at ordinal N | accepted row replaces all outgoing edges once; every rejection preserves the entire old edge set, not a prefix or empty graph. |
| `admission.open` | factory failure; wrong member id/dialect/owner; checkout failure; root-capture failure; graph conflict; exact success | on every failed row: no child map/root/generation/graph/pin publication. The output names whether caller keeps `member`, `pin`, or one bounded `abortTicket`. |
| `admission.register` | fresh unowned member; already-exact owner; foreign owner; id/dialect mismatch; same child/different slot; root capture failure | only valid fresh/exact rows may publish; fresh owner stamping happens only at commit; caller retains the unconsumed member on pre-capture rejection. |
| `admission.absorb` | no graph edge; wrong parent/slot; receipt member ref/owner mismatch; batch failure at each ordinal; 64 and 65 candidates | no empty-slot substitution or partial visible batch; 65 rejects before ownership transfer; failed batches report one bounded abort ticket per transferred candidate. |
| `genesis.cleanup` | factory fails at ordinal 0/N; graph rejects at ordinal 0/N; maximum 64 and 65 genesis specs | no receipt/visible graph publication on error; each successful earlier created member is retained by an abort ticket and retired exactly once, never raw-dropped. |

The Rust half should consume the same JSON rows for pure graph/admission outcomes, and add a small
observable fixture member that counts `retire_snapshot_read_erased` and `close_owned_step`. For
every failure ordinal it must show a stable old root/generation, no graph/map change, one close
sequence per transferred member, and terminal-empty retirement. A neutral corpus cannot certify
that last ownership property by itself.

### Candidate-root cleanup and ownership-returning API

The current root builder has a sharper boundary than an ordinary allocation failure.
[`ChildContentView::with_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8606)
first reads the member revision and mints an `ErasedSnapshotRead`, then calls the still-fallible
[`insert_entry`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8484).  Although
`ErasedSnapshotRead::Drop` does return the exact lease to its own registry
([`store/🦀️.rs`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:344)), that is
only a *returned-read notification*: the exact member's later maintenance/close must reclaim the
registry slot ([`return_snapshot_read`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:233)).
It is not a substitute for retaining an unreturnable member through bounded
`close_owned_step`.

Avoid relying on this destructor path as an admission rollback.  Split the existing view builder
without changing its immutable-root representation:

```text
ChildContentView::preflight_member(slot, child, dialect) -> ChildRootInsertAdmission
ChildContentView::capture_member(admission, &member) -> Result<PreparedChildRoot, Fault>
PreparedChildRoot::commit(self, prior_root) -> ChildContentView
```

`preflight_member` performs the identity-size/hash/probe/saturation decision against the current
root and records the exact insertion location. `capture_member` is the only async/fallible
snapshot-read point. `commit` asserts that the root still corresponds to the recorded admission
and writes the entry without another lookup or fallible operation. The app actor's exclusive
ownership makes that assertion the correct linearization rule. Thus a candidate snapshot can
never be stranded by a second `locate` failure after capture.

For failures *after* `PreparedChildRoot` exists, use a bounded owner—not an unbounded cleanup
queue—and reuse the two existing retirements by factoring one private single-member operation out
of `ChildContentRetirement`:

```text
ChildAdmissionRetirement<M> {
  key: (slot, child_id),
  root: Option<ChildContentRetirement>,
  member: Option<ChildMemberRetirement<M>>,
}
```

`root` is initialized from the one-entry candidate view. Its factored
`close_step_for_exact_member(key, &mut M, …)` must reject any entry whose key differs, retire the
erased snapshot with the existing `retire_snapshot_read_erased` path, and require its terminal
witness. Only then may the same `M` be moved into the existing `ChildMemberRetirement` and receive
its normal bounded `close_owned_step`. This preserves the real `ChildContentRetirement` semantics
and the real `ChildMemberRetirement` semantics; it does not expose a raw drop or invent a new
disposal protocol. A fixed `ArtifactFixedRegistry<ChildAdmissionRetirement<M>>` owns these values
while live maintenance and close advance exactly one ticket.

The owner-returning boundary should be explicit:

```text
register_child(..., member: M) -> Result<(), ChildMemberRegistrationError<M>>
```

may return `Some(member)` only before `capture_member` transfers a snapshot/membership cleanup
obligation. Once capture occurred, it must successfully insert the fixed abort ticket first;
otherwise it must have rejected before consuming `M`. `open_child` and `absorb_created_children`
have no caller-owned `M` result, so they must pre-admit that ticket **before** `M::open` or receipt
ownership transfer. Ticket exhaustion is a pre-transfer failure, never an implicit drop.

### Registration boundary

The existing registered `test-member-dialect-source` target only executes the current Bun source
fixture ([`framework-os Rust project`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json:36)); it is not a child-admission runtime gate. Add a dedicated kernel-owned
`child-admission-check` after the implementation lands. It must run the neutral oracle, list and
require exactly one FQN for each graph/admission/genesis law, then execute each exact law. Include
the real `open_child`, `register_child`, `absorb_created_children`, and
`dispatch_relation_group` paths—no helper-only substitute. Register its launch by seed then
generation/check-generated, not by editing generated launch output.

## Live source update — identity and graph authority partly closed

Current bytes now contain the planned source authority, but not the shared open/register/absorb
transaction. This supersedes only the two prior source observations about missing delegate and
same-parent/different-slot graph handling; it does not turn any runtime claim green.

- `SpaceMember` now declares synchronous cloned `artifact_ref() -> Option<ArtifactRef>` and
  `owner_ref() -> Option<OwnerRef>` at [`store/🦀️.rs:17293`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17293). Real `ArtifactStore`, `NoMembers`, the closed
  macro, and the test wrapper all delegate the new surface (17462, 17807, 17996, and 21267).
  The `Option` is correct as an expression of a malformed/no-dialect envelope, but every child
  admission consumer must reject `None`; it must never synthesize a reference from a caller key.
- [`OwnsAdmission`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18635) is a
  read-only exact ticket. `admit_owns` rejects different parent **or slot**, returns a false
  `inserts` ticket for an exact old edge, and `commit_owns_admitted` asserts the exact expected
  state (18695–18715). This closes the old overwrite defect for `insert_owns` and is a usable
  commit ticket for the pending app transaction. There is no rollback method because admission
  itself changes no graph state; that is preferable to the former broad `remove_owns(child)`.
- The neutral member-dialect corpus now carries seven graph vectors, including same-child/other-slot
  and both cycle shapes ([`🔣️.json`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json:3)). Its Bun oracle validates the fixture with AJV,
  independently builds the DAG in `graphlib`, and tests the closed edge-state rule
  ([`📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts:48)). The Rust law observes admission purity before commit
  ([`store/🦀️.rs:26285`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26285)). This is well-shaped source/test coverage, but I did **not** execute either target.

Two material REDs remain visible in the same current tree:

1. `sync_member` still erases the old outgoing `Owns` edges before performing per-child insertions
   ([`store/🦀️.rs:18773`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18773)). A
   later ownership/cycle failure produces a partial graph. Its candidate map needs a single
   preflight/replace operation; the one-edge ticket alone does not make a multi-edge projection
   atomic.
2. `open_child` and `register_child` still use immediate `insert_owns` after (or around) map
   insertion ([`plugin/🦀️.rs:19929`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19929),
   [`plugin/🦀️.rs:19972`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19972)).
   They do not read the new identity/owner facts, do not retain a prepared snapshot/member abort
   owner, and `absorb_created_children` still defaults a missing slot. The bounded shared
   transaction remains required.

## Live source update — single-child transaction source review

The preceding second RED is superseded for **single-child `open_child` and `register_child`**.
This review is source-only; no test or runtime command was run by this audit.

[`admit_child_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19886)
now makes every fallible decision before a member crosses an unreturnable boundary: child-root
index/hash, exact parent dialect, root retirement generation, exact `OwnsAdmission`, expected child
reference/owner, and child-map reservation. [`prepare_child_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19906)
then verifies the member's own reference and owner facts. Its root capture calls the new
preflight-index plus infallible insertion split (`ChildContentView::admit_member` / 
`capture_member_admitted`, 8491 and 8614), so there is no second lookup or allocation failure after
an erased snapshot has been issued. [`commit_child_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19925)
has only the owner stamp, graph-ticket commit, admitted map write, root swap, fixed retirement
write, generation write, and pin removal. Under the app's exclusive `&mut self` actor ownership,
those are the correct linearized no-failure commits.

For a post-open preparation failure, `open_child` first cancelled its exact map reservation and then
transfers the real member once into a pre-admitted fixed `ChildMemberRetirement` slot
([`plugin/🦀️.rs:19957`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19957)).
The 64-slot abort registry is now live-pumped at maintenance stage 20, close-pumped before child
root retirement, and included in both idle and terminal tests (23379, 23671, 23699, 24077). Since a
successful capture has no later fallible step, this owner correctly contains only the existing
`ChildMemberRetirement`; no candidate `ChildContentRetirement` is needed on this single-child
path.

Direct registration has been repaired to remain caller-pure. If a matching queued restore pin
exists, `prepare_child_member(..., restored = false)` rejects before `checkout`
([`plugin/🦀️.rs:19911`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19911));
the caller receives its unchanged `M`, and the pin remains queued. Only restored `open_child` may
apply the pin; its later failure transfers `M` to bounded abort retirement, rather than returning a
possibly checkout-mutated store.

`open_child` has no cancellation parameter. The current direct production reach is private
`load_child_pack`, invoked by the synchronous `AppCommand::LoadChildren` loop
([`plugin/🦀️.rs:24570`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24570),
[`plugin/🦀️.rs:31360`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31360)).
Therefore cancellation is an explicit nonclaim for this packet, not a demonstrated late-open race.
If a cancellable transport exposes it later, that route must supply a cancellation fence after
`M::open` and before `commit_child_member`, retaining the member in the same abort registry on a
lost authority.

Remaining scope REDs are intentionally outside the single-child landing: `LoadChildren` still
loads entries serially and can leave an earlier child committed when a later one fails; the
`absorb_created_children` receipt path and `dispatch_relation_group` genesis loop have not yet
adopted batch preflight/abort ownership; and `sync_member` remains a partial graph replacement.
Required exact runtime laws before acceptance are: failed open drains one abort owner with no
double-close; direct registration plus queued pin returns an unchanged member and preserves the
pin; open success consumes the pin once and publishes one exact map/graph/root generation; and
every later batch/genesis failure ordinal leaves zero visible candidates.

## Live source audit — member-factory fixture lifetime and `LoadChildren` cancellation

This is a current-byte source review only. I did not run the native law binary.

### Deterministic fixture RED

The narrow neutral loops now use the right bounded-owner fixture: `DemoSnapshot` supplies
`demo_closable_store_owners()` through `MemberStoreOwner` ([`store/🦀️.rs:21604`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21604)), and each successful
`RetainedTestMembers::create/open` result is closed by
`close_member_dialect_fixture` ([21748](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21748), [21781](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21781), [21819](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21819)).  The helper constrains every turn to one item / 4096 bytes and requires the terminal-empty witness.

Three current native test paths remain inconsistent with that owner discipline:

1. `typed_child_store_factory_round_trips_a_child_through_create_persist_open` creates both
   `child` and `reopened` but never closes either ([`store/🦀️.rs:26158`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26158)).
2. `member_factory_wrapper_cannot_bypass_exact_create_or_open_owner_catalog` likewise leaves
   `created` and `reopened` live ([26184](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26184)).
3. `member_close_rejects_missing_owner_and_preserves_the_installed_blocked_disposer` has become
   stale ([26203](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26203)): after it
   installs the now-closable `DemoSnapshot::member_store_owners()`, it still expects `Blocked`
   and returns without a close.  It needs either a deliberately blocked *separate* disposer
   fixture followed by its explicit terminal close, or a progress-to-terminal assertion for the
   closable catalog.

These are material, not cosmetic. `ArtifactStore::Drop` requires its full terminal-empty shallow
shell ([`store/🦀️.rs:16526`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16526)); a
successful test-owned store cannot be raw-dropped. Make the four successful stores mutable and
pass each exactly once through the existing bounded helper, after the last read assertion. For the
negative owner test, keep the pre-install `Err` assertion, then use an independently declared
blocked owner to establish `Blocked`, and still drive its disposer to terminal before scope exit.

### Production factory error disposition

The ordinary `Result` paths do **not** transfer a member-store owner before their last fallible
operation. `create_member_store` rejects empty/decode-invalid input before the envelope and then
awaits `ArtifactStore::new` before invoking the infallible owner installation
([`store/🦀️.rs:2973`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2973)).
`open_member_store` performs binary decode, parse, full id/schema/dialect/owner equality, and the
same fallible `ArtifactStore::new` before owner construction/install
([2992](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2992)).  `ArtifactStore::new`
itself returns all validation/history/fold/catalog errors before the final `Self` construction
([13740](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13740)); locals at those exits have
ordinary Rust ownership. `MemberStoreOwners::new` is nonfallible ([1971](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1971)), and
`install_member_store_owners_exact` is a fresh-store invariant assertion with no `Result` exit
([14147](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14147)).

Thus ordinary parse/store failure needs no member cleanup, and successful returns are the only
factory ownership transfer that needs a close. The install assertion is intentionally not a
recoverable production error; if it ever trips, ordinary cleanup is not established and the
terminal-drop assertion will abort. Treat it as an internal invariant, not evidence of a
recoverable install cleanup path.

### `LoadChildren` cancellation — explicit nonclaim

`AppCommand::LoadChildren` contains only `{ seq, entries }`, with no cancellation/generation or
deadline authority ([`spr/channel/🦀️.rs:1522`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1522)).
The plugin dispatcher holds its single `with_instances_mut` turn and calls
`resolve_ready(instance.app.load_child_pack(...))` sequentially for each entry
([`plugin/🦀️.rs:31352`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31352)); the
app path is merely `load_child_pack → open_child` ([24570](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24570)).
Accordingly there is no cancellation transition to audit or claim today: an entry either reaches
its synchronous-ready turn or returns a fault. The present semantic RED is serial partiality, not
a demonstrated cancellation leak—an earlier child can remain committed if a later parse/open
fails. A future cancellable transport must add authority at the batch boundary and prove that a
post-open lost authority transfers the member into the existing abort registry before
`commit_child_member`; it must not retrofit a raw drop. The current child-frame round-trip law
only proves wire encoding ([`spr/channel/🦀️.rs:2823`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2823)), not cancellation or batch atomicity.

## Live source update — fixture supersession and bounded `LoadChildren` P0

This is a second read-only current-byte review. No native binary was run by this audit.

### The three fixture-lifetime REDs are superseded

The prior three fixture findings have been repaired in current source and should not be carried
forward as live blockers:

- The factory round-trip now takes mutable `child` and `reopened` values through
  `close_member_dialect_fixture`, and the wrapper test does the same for `created` and `reopened`
  ([`store/🦀️.rs:26158`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26158),
  [`store/🦀️.rs:26184`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26184)).
- The former stale missing-owner law now proves the no-owner error first, installs
  `DemoSnapshot::member_store_owners()`, requires bounded progress, rejects catalog replacement,
  and uses the same terminal helper ([`store/🦀️.rs:26203`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26203)).
- The new plugin helpers drive actual `TestMembers::close_owned_step` and
  `VcsArtifactApp::close_step`, each at one item and exactly 4096 bytes, asserting the reported
  cap and terminal-empty witness ([`plugin/🦀️.rs:34815`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34815),
  [`plugin/🦀️.rs:34829`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34829)).
  Those are the production runtime's actual close grant values, not arbitrary test constants
  ([`plugin/🦀️.rs:28793`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28793),
  [`plugin/🦀️.rs:29131`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29131)).

The three new exact laws are sound **source-level** lifecycle subjects:

1. [`member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34844)
   covers factory rejection, post-construction queued-pin failure, bounded stage-20 retirement,
   and final app close.
2. [`member_factory_closed_dialect_register_rejects_pin_without_mutating_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34883)
   captures the pack before direct registration and proves returned caller ownership, no map/root/
   graph/generation publication, and bounded close.
3. [`member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34905)
   uses `TestMembers::create`, a real serialized envelope, a fresh `open_child`, exact graph/
   owner assertions, and bounded close of both apps.

`TestSnapshot` is not a mock member: it is the shared typed DSL/pack snapshot fixture
([`test-app-mutations/document/🦀️.rs:15`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🦀️.rs:15))
and its catalog supplies the real `ArtifactStoreCursorDisposer` path. The laws therefore exercise
the actual close protocol, but their default snapshots do not include an intentionally non-empty
history/read-retirement payload. That is a coverage addition, not a defect in their current
factory/lifecycle assertion. They remain unexecuted by this audit, so neither is runtime evidence.

### Blocking inbound RED: tag 14 has no live paged route

`LoadChildren` cannot presently claim a real plugin ingress at all. The production ingress creates
`PagedAppCommandDecodeCursor` from `PluginCommandIngress::Encoded`
([`plugin/🦀️.rs:30610`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30610)); its header state accepts
tags 0–4, 6–9, 15, 16, 27 and 29, but **not 14**, returning
`plugin.command-route-state-machine-required` ([`spr/channel/🦀️.rs:1255`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1255)).
The apparent `LoadChildren` dispatcher loop is consequently unreachable through ordinary encoded
transport ([`plugin/🦀️.rs:31352`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31352)). The old whole-frame test reaches only
`decode_app_command`, not the retained ingress ([`spr/channel/🦀️.rs:2231`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2231),
[`spr/channel/🦀️.rs:2823`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2823)). It is not runtime proof.

The helper decoder is also unsafe as a future authority: it casts an attacker-controlled count to
`Vec::with_capacity(count as usize)` then reads unconstrained strings/bytes
([`spr/channel/🦀️.rs:2194`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2194)). The nominal `AppCommand` still exposes an
unbounded `Vec<ChildPackEntry>` ([`spr/channel/🦀️.rs:1522`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1522)). This is an authority and
availability RED, not merely missing test coverage.

### Minimal schema-first bounded transaction

P0 must be a single feature, ordered as follows.

1. Define a retained tag-14 `ChildPackLoadBatchV1` decoder over `PagedCommand`, rather than
   widening `read_vec_child_pack`. It must reject before allocation: more than 64 entries, more
   than the existing 64 pages / 262144 bytes, oversized slot/child/dialect fields, duplicate
   `(slot, child_id)`, duplicate child identity across slots, malformed dialect, truncation, and
   a non-exhausted trailing reader. `COMMAND_BATCH_MAXIMUM_ITEMS = 64` and the paged 4096-byte / 64
   page limits are already protocol authority ([`spr/channel/🦀️.rs:46`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:46)).
   Give the decoder a terminal `close_step` and install it in `PluginCommandIngress`; a tag-14
   frame must be decoded from `Encoded(PagedCommand)`, not from the whole-buffer helper.
2. Add one `PluginApp::load_child_packs_batch` boundary and change the route to call it once.
   The existing trait advertises only per-entry `load_child_pack` ([`plugin/🦀️.rs:11785](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11785)); its dispatcher loop
   commits an earlier child before a later failure. A one-entry convenience may delegate to the
   batch primitive, but the route must never loop a single-child commit.
3. The batch must pre-reserve all resource authority before any `M::open`: every child-map ticket,
   root target, graph edge, and a fixed abort owner. Sixty-four is the honest P0 ceiling because
   the actual abort registry has 64 slots ([`plugin/🦀️.rs:15471`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15471)); accepting the
   larger 1024 child-root capacity would create a cleanup obligation that current maintenance
   cannot own. Preflight must use a staged occupancy/graph view which includes prior candidates;
   invoking one-edge `admit_owns` against only live graph state is insufficient
   ([`store/🦀️.rs:18695`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18695)).
4. After all reservations, open/validate/capture every member into a staged root. If any ordinal
   fails, transfer every constructed member and captured root to a fixed, live-pumped batch abort
   owner composed from the existing `ChildMemberRetirement` and `ChildContentRetirement`; do not
   raw-drop or publish an intermediate root. The current single-child success property is not
   enough: `capture_member_admitted` owns a read/snapshot result and the current abort owner only
   contains a member. Commit only after all work succeeds, in a no-failure, no-await critical
   phase: stamp exact owners, commit all graph tickets, map tickets and one root swap, advance one
   generation, then consume pins. Current single-child `commit_child_member` is async
   ([`plugin/🦀️.rs:19925`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19925)); batch design must not infer its awaits are an atomic
   multi-child commit. Acquire/admit the required actors before staging or introduce an explicit
   no-fail commit guard.

Current source has no route cancellation authority after admission: `seq` is a wire correlation,
not authority. That is a **current RED**, not permission for P0 to omit cancellation. The P0
receipt must therefore introduce an explicit cancel/generation owner before preflight, close an
incomplete or cancelled encoded command without publication, and prove cancellation before commit
leaves every plane unchanged. It must also prove that cancellation cannot interleave the no-await
commit phase. A later genuinely asynchronous factory-open path needs the same owner on its retained
open cursor; it cannot reuse `seq` or retain an arbitrary future as implied authority.

### Required neutral and registered acceptance

Create a neutral schema/corpus plus independent Bun/AJV and byte-decoder oracle for: two valid
noncolliding children; zero and 64 entries; 65 entries; 257-byte identity; malformed dialect;
truncated/over-limit envelope; duplicate pair; same child in different slots; pre-existing slot/
child collision; each factory/identity/pin/capture failure ordinal; ingress cancellation while
decoding; and success with one generation plus stable ordered child roots. Every failure vector
must observe zero child-map, graph, root, generation and pin consumption, then exact bounded abort
drain. Add a project script that runs the neutral oracle, list/requires exactly one FQN for the
retained tag-14 route and each batch law, then executes those FQNs. It should include a real
`PluginCommandIngress::Encoded(PagedCommand)` test; the helper-only round trip is expressly
insufficient. Register launch from the seed and regenerate/check output. `ReadChildren` outbound
pagination is a separate packet: current `child_packs()` builds every live entry, so P0 must not
claim a bounded full-catalog response merely because inbound batch is 64.

## Exact tag-14 retained-ingress and batch-receipt blueprint

This section narrows the preceding P0 to the current authority surfaces. It is a design map from
current source, not an implementation or runtime result.

### Canonical wire authority and its limits

There is no physical JSON schema for this protocol today. The canonical source is the typed
`AppCommand::LoadChildren` declaration and its command encoder/whole-buffer decoder
([`spr/channel/🦀️.rs:1522`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1522),
[`spr/channel/🦀️.rs:2078`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2078),
[`spr/channel/🦀️.rs:2194`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2194)). Tag 14's current
layout is `tag | sequence | count | (slot, child-id, dialect, envelope-pack)*`, each variable
field length-prefixed. The canonical cross-language drift guard is also embedded in that Rust
module: its one-child `LoadChildren` specimen and golden `0e...` byte string live at
[`spr/channel/🦀️.rs:3040`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:3040) and
[`spr/channel/🦀️.rs:3167`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:3167). It is a golden one-page
codec fixture, not a schema or retained ingress law.

The transport already establishes these hard raw limits:

| Current authority | Value | Source consequence |
| --- | ---: | --- |
| `COMMAND_PAGE_MAXIMUM_BYTES` | 4096 | one releasable raw-page grant |
| `COMMAND_MAXIMUM_PAGES` | 64 | one command has at most 64 raw pages |
| `COMMAND_MAXIMUM_BYTES` | 262144 | aggregate raw command ceiling |
| `COMMAND_BATCH_MAXIMUM_ITEMS` | 64 | the reusable exact batch ceiling, but tag 14 does not yet use it |
| child slot/id | 256 bytes each | `ChildContentView` admission bound, not current wire validation |
| child roots/members | 1024 | storage capacity, not safe batch/abort capacity |
| failed-admission retirement | 64 | actual maximum P0 batch cardinality |

The first four are defined by [`spr/channel/🦀️.rs:46`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:46);
the identity and root constants are at [`plugin/🦀️.rs:8175`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8175),
and the actual 64-slot retirement owner is at [`plugin/🦀️.rs:15471`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15471). `PagedCommand.item_count` is
only populated by the special Presence constructor; it is zero for ordinary tag-14 pages
([`spr/channel/🦀️.rs:177`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:177)). A child route must decode and
validate the on-wire count itself; it may not treat generic metadata as an admission proof.

Create a new canonical, language-neutral fixture rather than expanding the accidental whole-frame
golden table: proposed
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🧸load-children-batch/🧬️schema/🔣️.json`
and sibling `🧪️fixture/🔣️.json`. The schema should name `semio.app-command.load-children/v1`, tag
14, the five limits above, expected terminal wire result, and a page-vector form—not claim that
the Rust enum is itself a cross-language schema. The fixture's independent Bun/AJV/byte-reader
oracle belongs in the existing plugin package runner
[`plugin/📦️packages/🦀️rust/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts), because that
package owns the live ingress/route. It must not edit or rely on generated launch output.

### Required retained route, rather than a larger generic `AppCommand`

Do **not** add tag 14 to `PagedAppCommandDecodeState` and return another unbounded
`DecodedAppCommandOwner`. That owner currently only has close cases for ordinary fields; route
commands are explicitly unreachable ([`spr/channel/🦀️.rs:1206`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1206),
[`spr/channel/🦀️.rs:1374`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1374)). A decoded 262144-byte child pack
would also be unable to satisfy the existing 4096-byte ingress-close grant if retained as one
`Vec<u8>`.

The correct existing pattern is a route-specific retained owner, as Presence demonstrates:
`PresenceCommandCursor` validates its own page/item shape and releases its raw page under a grant
([`spr/channel/🦀️.rs:1081`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1081)); the app reserves its publication/retirement
slots before accepting the cursor ([`plugin/🦀️.rs:24518`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24518)); and its publication retains
raw input, cancellation authority, candidate state and bounded close until terminal
([`plugin/🦀️.rs:9168`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9168),
[`plugin/🦀️.rs:9374`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9374)).

For tag 14, add a distinct `ChildPackLoadCursor` in `spr/channel`, and matching
`PluginCommandIngress::{LoadChildren, ClosingLoadChildren}` states in
[`plugin/🦀️.rs:30610`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30610). `Encoded(PagedCommand)` must branch on `kind == 14`
before generic cursor construction. The cursor must retain raw fixed pages, header/count,
bounded identity metadata and page spans. It must not flatten every envelope into a retained
`Vec<ChildPackEntry>`.

The reason is material: the current `MemberFactory::open` accepts only a contiguous `&[u8]`
([`store/🦀️.rs:18122`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18122)), while a full document envelope may validly use
the existing 262144-byte decode ceiling ([`store/🦀️.rs:7543`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:7543)). A retained giant `Vec` cannot be
honestly released under one 4096-byte close grant. The P0 cursor therefore needs page-span
metadata over the original bounded `PagedCommand`, then a **single transient** contiguous scratch
copy for one `M::open`, dropped before it advances or yields. If factory open can yield, the
current `async fn open` API has no retainable factory-open owner: the live dispatcher merely
bridges it with `resolve_ready` ([`plugin/🦀️.rs:31352`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31352)). In that case a retained,
page-backed `MemberOpenCursor` is a prerequisite; silently holding a pending arbitrary future or
an oversized `Vec` is not a bounded P0 implementation.

### Receipt lifecycle and linearization

`PluginExchangeOutput.retry_command` already retains an ingress at the same envelope sequence
([`plugin/🦀️.rs:30713`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30713)); use that established owner rather than a parallel untracked task. A
`ChildLoadBatchReceipt` should have exactly these phases:

1. `Scan`: cursor validates tag/sequence/count, all field lengths, UTF-8 and exact EOF; it owns
   raw pages and at most 64 fixed identity/span entries. Cancellation here only moves to
   `CloseRaw`; no app reservation or publication exists.
2. `Preflight`: from the complete span table, reserve the fixed batch-abort ticket, all exact
   child-map admissions, all root placements, all queued-pin decisions, and **one** graph batch
   ticket before any `M::open`. Rejection cancels every reservation and moves directly to
   `CloseRaw` with zero app mutation.
3. `OpenAndCapture`: each member is opened from one transient scratch pack, then exact
   ref/owner/pin and snapshot capture are staged. Any fault or cancellation moves every opened
   member plus captured candidate root into one fixed batch-retirement owner; it does not return a
   partly-open member to a protocol peer.
4. `Commit`: after every open/capture succeeds, perform only synchronous no-fail ticket writes:
   owner stamps, graph batch replacement, member-map inserts, one child-root swap, one generation
   advance and pin removal. Cancellation cannot interleave this critical section. Only then is the
   batch logically accepted.
5. `CloseRaw`/`Abort`: release at most one raw page or one bounded retirement turn per poll,
   requiring terminal-empty witness before removal. A failure emits one `AppFrame::Error` only
   after abort/raw closure. A success emits `Done` only after input ownership is terminal. There
   is never an early `Done`, a half batch, or a raw `Drop`.

This needs an ingress step type able to retain and drive the receipt from `plugin_exchange`, not
the current `PluginCommandIngressStep::Ready(DecodedAppCommandOwner)` alone
([`plugin/🦀️.rs:30618`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30618),
[`plugin/🦀️.rs:31041`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31041)). This is the narrow needed production boundary;
it avoids a separate task, inferred cancellation authority, and the current serial direct loop.

### Graph-batch prerequisite and current source audit

The new single-edge `OwnsAdmission` correctly binds its private `Arc` and generation to its
originating graph at commit ([`store/🦀️.rs:18637`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18637),
[`store/🦀️.rs:18712`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18712)). That is source-correct for one edge, but it deliberately
prevents a naïve batch: every insert admission made at the same graph generation becomes stale
after the first commit advances it. `ChildLoadBatchReceipt` therefore requires a dedicated
`OwnsBatchAdmission`: build a shadow owned/link map, validate all incoming identifiers, existing
foreign ownership, batch duplicates and cycles, reserve map capacity, then replace/advance once.

Current [`CompositionGraph::sync_member`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18784) is a good **source-only** model for that
graph portion. It builds candidate owned/link maps; rejects target identity mismatch, foreign
owner, multi-slot duplicate child, own/link cycles; reserves before the sole mutation; then
replaces outgoing state and advances generation once. The current seven-row schema/corpus and
Graphlib oracle explicitly distinguish insertion from full replacement
([`member-dialect schema`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/🧬️schema/🔣️.json:22),
[`fixture`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json:3),
[`oracle`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts:67)). I found no source
partial-publication path in that replacement routine: all awaits precede `retain`/`extend`.

**Schema authority now confirmed:** this is intentionally not duplicate-*slot* rejection.
`ChildSlotSpec` owns `many: bool`
([`schema/component.rs:127`](../../../../../../🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:127)), and
the public composition contract expressly says a `many` slot contributes more than one pair for
the same slot kind ([`plugin/🦀️.rs:994`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:994)).
Thus `next_owns` is correctly keyed by child identity: the committed
`graph-second-child-same-slot` row remains `syncAccepted: true`
([`fixture:10`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json:10)), while one child in multiple
slots/parents is rejected. This closes the graph-only question. The future tag-14 route has a
separate P0 obligation to load the parent's declared `ChildSlotSpec`: reject an undeclared slot,
kind mismatch, and a second child only when that exact declared slot has `many == false`; permit
distinct children only for `many == true`. The generic graph cannot enforce that policy because it
does not own a parent's declaration.

It is not itself the child-load transaction: it obtains refs from a snapshot and mutates only the
graph, whereas the P0 receipt must bind the same candidate identities to member-map/root/abort
admissions and commit every plane together. Reuse its staged-validation algorithm in
`OwnsBatchAdmission`; do not invoke `sync_member` after independently publishing children.

### Declared-parent restore authority — current RED

The current child reload has exact envelope and ownership checks, but it has no authority tying
an incoming child set to the already-loaded parent snapshot. `VcsArtifactApp::open_child` builds
both its expected child identity and `OwnerRef` only from the incoming `(slot, child_id, dialect)`
plus its own parent envelope; `PluginApp::load_child_pack` is then a direct single-child call to
that method ([`plugin/🦀️.rs:19884`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19884),
[`plugin/🦀️.rs:24570`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24570)). A
valid owned envelope for an otherwise undeclared child is therefore admissible today. The
protocol's serial loader repeats that bypass for every entry
([`plugin/🦀️.rs:31360`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31360)).

The schema and value authorities already exist but are not connected in production. Derived
`ArtifactCompositionFields::child_slots()` yields each declared `(name, kind, many)` table
([`schema/component.rs:147`](../../../../../../🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:147)); a
persisted `ArtifactChild` projects the complete stored child identity as `ChildRef { slot,
child_id, target }` through `to_child_ref`
([`store/🦀️.rs:2725`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2725)).
`ArtifactRefs::child_refs()` is precisely the homogeneous snapshot seam, but current production
has no implementation; the sole graph-sync path states this explicitly and remains opt-in
([`store/🦀️.rs:2832`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2832),
[`plugin/🦀️.rs:20983`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20983)).
Builder `.composition::<Snapshot>()` captures only static metadata today; it is not a live
snapshot projection ([`plugin/🦀️.rs:3317`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:3317)).

The smallest honest P0 is a kernel `ChildRestoreProjection`, created from the loaded parent
snapshot's exact `ArtifactRefs` values and its derived `ChildSlotSpec` table. It must sort and
validate: declared slot; declared kind equals the full target dialect kind; `target.artifact_id ==
child_id`; at most one value for a `many == false` slot; no duplicate child id across slots; and
the full target dialect, not merely schema/id. It returns an error by default rather than an
empty or inferred projection. A leaf can return a verified empty projection.

`ArtifactApp` should expose that explicit projection boundary, with the seven direct apps and the
`EditorApp`/`ViewerApp` adapters forwarding it. The latter are the correct application seams:
their public editor/viewer roots are synchronous ([`plugin/🦀️.rs:25982`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25982),
[`plugin/🦀️.rs:26327`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26327));
they must not gain an ambient default, a synthetic document dialect, or a generic blocking async
bridge. The generic helper may use each concrete snapshot's static schema table, but only the
loaded snapshot produces the dynamic child values.

The tag-14 batch route must obtain this projection after parent `LoadDocument`, require exact set
equality with wire `(slot, child_id, full target)` entries before calling any `M::open`, then use
the existing envelope/`OwnerRef` check as its second proof. An entry may not declare a child by
claiming its own owner. A pre-existing child root requires a distinct replacement receipt (P0 can
fail closed); `register_child` / prepared genesis stays a separate locally authorized creation
path, never a wire-restore substitute.

Required neutral/Rust rows add: undeclared slot; declared-kind mismatch; same schema/id but
different full dialect; target-id mismatch; wire omission of a declared child; wire extra child;
two values in singular slot; two distinct values in `many` slot; one child claimed through two
slots; foreign or mismatched envelope owner; cancellation and every preflight/open failure with
unchanged maps/root/pins/graph generation. Positive rows must exercise one direct app plus both
editor and viewer adapters, proving the projection is not fixture-only. This remains a source RED
until a batch route and these laws land.

### Exact P0 corpus and gate

The new fixture needs byte/page vectors for: empty; one; two colliding-hash but distinct children;
64; 65; 257-byte slot and child id; malformed UTF-8; malformed dialect; overlong/truncated varint;
truncated field/page; trailing byte; envelope span beyond raw extent; duplicate exact pair; one
child in two slots; pre-existing root/member collision; foreign owner; every open/identity/pin/
capture failure ordinal; cancel while scanning; cancel after preflight; cancel after the first
open; and a successful 64-entry commit. Each row records raw-page releases, retained candidate
count, abort turns, expected graph/root/map/pin generations and exactly one terminal reply.

The independent oracle should parse the page vectors with a fresh byte reader and use AJV for the
schema. It should additionally model the single replacement graph with Graphlib; it must not
call Rust codec helpers. The Rust exact laws must cover actual `PluginCommandIngress::Encoded`,
the `retry_command` route, `VcsArtifactApp` member open/capture, batch graph ticket, bounded abort
and terminal close. Register an `os-plugin:child-load-batch-check` style target in the existing
package `📜️script.ts`: oracle first, exact-one FQN preflight for every law, then exact execution;
add its launch entry only by seed then generator/check. No full `ReadChildren` response, browser
consumer, or generalized async member-factory rewrite is accepted by this P0 unless it is
separately implemented and proved.

## Kernel Fixture-Sweep Dependency Frontier

### Current source diagnosis

The focused kernel runner nevertheless compiles the entire fleet. Its native route invokes
`runCargoTestBudgeted(["semio-framework-os-kernel"], ..., ["--lib", "--features",
"sync,ureq", ...])` in
[`os kernel 📜️script.ts:27`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:27).
Cargo therefore resolves the kernel's complete `[dev-dependencies]` set even though the fleet body
is guarded by `#[cfg(all(test, feature = "dsl-fixture-sweep-full"))]`
([`fixture-sweep/🦀️.rs:25`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs:25)).
That feature does not make dev-dependencies optional to Cargo.

The kernel manifest has 28 sweep-only dev edges after the async test macro: 27 external/plugin-or-app
crates (`block`, `cad_document`, `dag_app`, `draw`, `fem`, `gis`, `home`, `imperative`, `layout`,
`lowpoly`, `mathematical`, `norm`, `note_app`, `presentation`, `procedural`, `process_3d`, `puzzle`,
`raster`, `reasoning_mindmap_plugin`, `remodel`, `sequence`, `shooting`, `sourcing`, `trinity`,
`vcs_app`, `writer`, plus framework `flow_app`) and the whole native OS host
`semio_framework_os` ([`os-kernel Cargo.toml:131`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:131)).
The OS-host edge alone traverses plugin host/WGPU/native presentation through
[`os host Cargo.toml:23`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml:23).
This explains a focused store-member law reaching Wasmtime/Cranelift: it is a test-graph
misplacement, not a kernel-law dependency.

The only consumer is the aggregating DSL example sweep:
[`fixture-sweep/🦀️.rs:32`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs:32)
imports every concrete snapshot; `registry()` maps roughly 50 real artifact spellings to
`ArtifactDsl` checks ([`fixture-sweep/🦀️.rs:98`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs:98)).
It additionally imports `WorkflowSnapshot`, `SpaceSnapshot`, and `CollectionSnapshot` from the
full OS host at lines 83--90. The module uses only public kernel `os_store`/`os_dsl` test support,
repo-root discovery, and concrete app types; it has no semantic reason to compile *inside* the
kernel. The kernel mounts it solely at
[`os-kernel/🦀️.rs:100`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:100).

### Small clean separation

Create a dedicated workspace test package under the fixture-sweep taxonomy, for example
`🔨️modules/🗣️dsl/🧪️fixture-sweep/📦️packages/🦀️rust`, named
`semio-framework-os-dsl-fixture-sweep`. It is a **test-role leaf**, not a kernel feature or an
OS-host fixture. Its normal dependencies are the kernel (for public `ArtifactDsl`, parsing and
test support), the 27 concrete providers, framework flow, and the OS host only for the three host
snapshots. Those edges all point from test leaf to production leaves; none points back to the test
leaf, so this removes rather than hides the framework-to-plugin dev fan-in and creates no optional
runtime/cyclic dependency.

Move **only** the first, fleet-owning `#[cfg(all(test, feature = "dsl-fixture-sweep-full"))] mod
tests` (lines 25--383) to that package's test binary, changing its `crate::` references to
`semio_framework_os_kernel::`. Keep repository fixture discovery and the actual registry rows
intact; no soft skip or subset reduction is justified. Keep the kernel's mount of the same source
file: its later M5 grammar/protocol modules are kernel-only and still use the async test macro.
Remove the feature and every fleet dev edge, but retain `semio-framework-async-macros` in the
kernel and add it to the dedicated test package for the two migrated async laws. This makes the
existing seven-law
`member-dialect-check` compile only the kernel/plugin subjects it explicitly names
([`os kernel 📜️script.ts:70`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:70)).

The repository test host is deliberately dependency-free
([`repo test host Cargo.toml:1`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml:1)); it is not a valid place to
link concrete production implementations. The new package should instead be a workspace member
with its own `📜️script.ts` target, invoking `cargo test -p semio-framework-os-dsl-fixture-sweep
--test fixture_sweep` (or its one declared integration target) only. Its Nx project must route
only through that script, and the root test taxonomy should select it for the former full
`test dsl`/exhaustive conformance phase. Fast kernel targets must not transitively invoke it.

### Preservation and acceptance

Before removal, establish an exact static registry law in the new package: the same labels/envelope
ids and fixture discovery outcomes must be present, including the three OS-host snapshots. Then
run the migrated exhaustive package once and retain its reported counts of directories, fixture
files, registered kinds, laws and unmapped rows as the baseline; an empty, silently reduced or
all-soft-skipped registry fails. The existing per-artifact byte/DSL laws remain the production
authority; this fleet test is their real-example conformance consumer, not a replacement.

Register `dsl-fixture-sweep-check`: first a language-neutral inventory/oracle of the expected
registry rows and fixture spellings, then exact-one test selection and the dedicated test binary.
Add its launch entry only through the seed followed by generation/check. The lightweight kernel
`member-dialect-check` remains the iteration gate; the fleet sweep becomes the independent,
explicitly expensive conformance gate. No Cargo run was started for this audit.

## Host Synchronous-Facade Compile Frontier

### Classification from current source

The coordinator's `metadata32064` result is a source-compile RED, not runtime evidence: the
`semio-framework-os` library reached its host source and stopped at the native async transition
(including `Box<dyn Backbone>` at host line 910, file/folder constructors at 2293/2299, async
storage calls in the `2316..2357` range, `ArtifactHost` calls at 2478--2480, and the missing
`MediaContract: ToValue` at 3864). I did not start another Cargo command while the seven-law
native slot is owned elsewhere.

Two sections are demonstrably unused production facades and should be removed rather than
given another synchronous poll bridge.

- [`OsWorkflowStore:786`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:786) has no
  non-host Rust caller; all matches outside its implementation are commentary, re-exports, or
  its own tests. Its native [`attach_backbone:910`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:910)
  is consequently the first stale `Box<dyn Backbone>` boundary. Delete this wrapper, its
  wrapper-only tests (including the two-backbone fixture at line 1882), the exclusive
  `OsSpaceStore` alias/re-export, and stale comments together. The live WGPU shell does not
  construct it.
- [`host_runtime:2409`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2409) also has
  no production import/call. Its synchronous [`open_document:2477`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2477)
  calls now-async `ArtifactHost::open` and `subscribe`; remove the module and its local test.
  The actual WGPU owner opens and subscribes directly with `await` in
  [`Shell WGPU:3686`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3686),
  so retaining this helper would duplicate and misrepresent the real ownership path.

The remaining facade is live and cannot be deleted or merely retyped without a caller packet.
[`OsBackbonePort:930`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:930) performs a
one-poll `resolve_kernel_future` bridge over the now-async store port. `SpaceBackbonePort` then
implements those synchronous reads/writes at
[`host:2280`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2280), including the
failing file/folder construction and storage calls; the host re-exports its constructors at
[`2396--2402`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2396). The Space Home
commands invoke them directly:

- [`create-studio:21`](../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏙️create-studio/🦀️.rs:21)
  creates a folder port and catalog, then emits navigation; and
- [`bind-space-file:22`](../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏙️bind-space-file/🦀️.rs:22)
  creates a file port, writes it, and suppresses every error in its public handler at line 44.

Both command paths additionally use `resolve_ready` and a process-global
`shared_studio_ports` registry ([`space plugin:166`](../../../../../../✏️s/🔌️plugins/🪐️space/🦀️.rs:166)).
Thus an async trait signature plus another `resolve_kernel_future`/`resolve_ready` compatibility
layer would still be a false immediate-ready claim, lose cancellation, and leave an unowned
cross-window registry. This is a production RED, independent of the two deletable facades.

The `ToValue` failure is a separate truthful UI-contract defect. `MediaContract` deliberately
has no such codec ([`workflow:39`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs:39)),
yet the live workflow window calls `os_workflow_to_node_graph_payload` at
[`space workflow window:106`](../../../../../../✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs:106),
which tries `edge.contract.to_value()` ([`host:3864`](../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3864)).
An import cannot fix this. Either make the workflow module's codec an explicit schema-owned
contract or, more narrowly, replace this presentation-only call with an explicit node-graph
projection of `kind_id`, tagged media form/wire, and optional conversion. Do not derive serde or
`ToValue` as an accidental persistence contract. Cover document/binary wire and both absent and
present conversion in the window payload law.

### Dependency-ordered correction

1. Remove `OsWorkflowStore`/`host_runtime` and their re-exports/tests in one source-owned cleanup,
   then implement the explicit `MediaContract` presentation projection. This removes dead
   compiler fronts without changing a live native flow.
2. Replace Home's synchronous folder/file commands with a native-shell-owned asynchronous
   persistence operation: it carries the requesting window/space identity and selected path,
   opens the path asynchronously, writes or creates the complete document set, reports
   progress/cancellation, and emits a success-or-failure receipt. Only a successful durable
   receipt may advance Home catalog generation/navigate; failure must be observable and leave the
   draft/catalog unmodified.
3. Make that operation's `ArtifactHost`/WGPU document lifecycle own the per-space port. Retire
   `OsBackbonePort`, `OsBackbonePorts`, `SpaceBackbonePort`, the `open_*_space_backbone` exports,
   host synchronous catalog helpers, and `shared_studio_ports`; never substitute another global
   map. The WGPU shell's awaited document lifecycle is the appropriate existing owner, not the
   compatibility bridge.

The focused proof must exercise a real folder/file persistence attempt, one cancellation before
write, write/open failure with no navigation or catalog publication, and one success with exact
reopen bytes through the same owned host session. It also needs two simultaneous-window attempts
to prove no port crosses space/window identity. Register it as a separate native host/Home
operation target: exact-one Rust selection, independent neutral byte/catalog oracle first, then
the actual native fixture; seed then generate its launch entry. A compile-only pass or a
`resolve_kernel_future`-backed mock is not acceptance.

The current WGPU shell has a useful lower-level scheduler but not an admissible persistence
contract. `submit_shell_io_future` is bounded at 64 pending tasks
([`Shell WGPU:2057`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2057))
and already awaits file/folder dialogs. Its current `requestFileSave` path, however, writes raw
bytes and then re-dispatches `bindSpaceFile`
([`Shell WGPU:4357`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4357));
that command repeats synchronous persistence and discards failure. It is a migration substrate,
not the desired operation. Likewise `ReplayShellCommand` is unsuitable: the handler accepts only
directory commands and the two open-artifact ids at
[`Shell WGPU:4044`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4044);
other ids are intentional no-ops. The replacement requires a typed kernel/shell persistence
request plus a typed success/failure/cancel receipt correlated to the Home command. It must not
smuggle a new raw action id through `ReplayShellCommand` or path-bearing JSON operation fields.

The scheduler cannot be reused unchanged. `PendingShellIo` holds only a receiver and optional
task ([`Shell WGPU:1505`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1505));
it has no request/session/generation identity. Full capacity silently drops the request at
[`2057--2060`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2057),
and the poller examines only the queue front and breaks when it is pending
([`2070--2094`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2070)),
so a file dialog head-of-line blocks completed later work. It cancels only on broad shell teardown.
The persistence operation therefore needs its own exact id, scope generation and cancel token;
terminal receipt for success, denial, capacity rejection, cancellation and failure; bounded fair
completion routing; and unmount cleanup. The shell's worker pool/dialog primitives may be used
inside that retained operation, but `ShellIoCompletion::Actions` is not its protocol.

The source census expands that packet beyond `🖥️host`. The framework Space module independently
declares synchronous `SpaceBackbonePort`
([`framework space:1775`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs:1775))
and directly calls `crate::host::resolve_kernel_future` in its blanket implementation at line
1783. The installed Space plugin uses it for draft/catalog reads and writes at lines 227, 235,
279 and 431. Its `DraftCatalog`'s promote/demote/expiry byte moves consequently remain synchronous
authority too. More public command routes use the old host helpers and hide failure: Home import
at [`import-space:21`](../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏙️import-space/🦀️.rs:21),
delete at [`delete-vfs-node:26`](../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️delete-virtual-file-system-node/🦀️.rs:26),
and engine pack import at
[`import-space-pack-payload:24`](../../../../../../✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/💾️import-space-pack-payload/🦀️.rs:24).

The native persistence packet must therefore convert the host catalog layer **and** this
framework-space draft/catalog byte-move seam into the same retained async operation ownership,
before removing their two synchronous bridges. It needs to replace create, bind, import and delete
with typed request/terminal-receipt routes that mutate Home's catalog generation only after the
durable result. Removing just host `OsBackbonePort` would merely expose the next unsupported
`SpaceBackbonePort` bridge; retyping only the ports would leave the current swallowed-error and
pre-success-publication violations intact.

## Schema-Derived Parent Restore Projection

The proposed allocation-free, borrowed schema visitor is the right dependency direction: schema
may own primitive borrowed coordinate fields and a visitor trait, while `🏪️store` implements that
trait for its owned `ArtifactChild<S>`. It must not import `ArtifactRef`, `Dialect`, or a store
handle into schema. `ArtifactChild` already has precisely the wire identity needed for the
implementation — `child_id` plus `target` ([`store:2651`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2651)) — and the
schema visitor can expose those as borrowed primitive id/kind/standard/subset fields. `Option<T>`
delegates with the same cardinality and `Vec<T>` delegates with `MANY = true`; neither copies a
child payload. The snapshot derive supplies the field/slot name to each callback, because the
child itself correctly does not know its parent field name.

The important correction is that `#[child(...)]` must become the source of truth for **both** the
static `ChildSlotSpec` and the value walk. Today the derive only classifies a syntactic direct
`ArtifactChild`, one `Option`, or `Vec` by last path segment
([`schema derive:92`](../../../../../../🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs:92)); a
field with a child attribute but an alias is silently `None`. That is already a live mismatch:
Energy's `structure`/`zones` are aliases at
[`energy schema:45`](../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:45),
and Norm EN1990's `q_k` is another at
[`Norm EN1990 schema:20`](../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:20).
The present unwrapping also stops after one `Option`, whereas installed stdio shapes contain
`Option<Option<ArtifactChild<...>>>`. Keeping syntactic detection would perpetuate an
undeclared/undiscoverable child restore path.

Therefore the derive should emit, for every `#[child(kind = "…")]` field, (1) its static slot
with `many` supplied by an associated `ChildFieldRefs::MANY` constant and (2) its borrowed
visitor call. A wrongly annotated field then fails to compile for lack of `ChildFieldRefs`, rather
than disappearing from the manifest. It should reject `#[child]` without the required kind, just
as it does today for directly recognized children. Unannotated fields remain outside composition
authority. This correctly resolves type aliases, qualified `std::option::Option`, nested option
forms and vectors without a framework-to-schema dependency.

Make the adjacent `ArtifactCompositionFields::{child_slots,link_slots}` synchronous in this same
schema packet. Both return static derived slices, yet their current `impl Future` signatures force
callers to cross a synchronous bridge; the only current direct consumers are the store fixture,
the derive output, and the plugin builder's `resolve_ready` use. A static trait method makes the
restore hook honest without creating an executor dependency in schema or reviving a blocking host
facade.

`VcsArtifactApp::open_child` must not add this as a universal `A::Snapshot` bound: `ArtifactApp`
currently serves many leaf snapshot types and schema's default empty composition trait would turn
a generic bound into an accidental empty grant. Instead add an explicit, fail-closed
`child_restore_projection(&self or &snapshot) -> Result<...>` hook on the app boundary, forwarding
through `EditorApp` and `ViewerApp`. A non-composed app returns an explicit denial. A composed
implementation invokes its derived value visitor plus its own
`ArtifactCompositionFields::child_slots` table ([`schema component:147`](../../../../../../🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:147)),
then validates the exact incoming wire set, slot kind and `many` policy, id/dialect, and parent
full dialect before `M::open`. Only that projection may authorize the existing child-admission
sequence at [`plugin:19957`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19957).

The focused corpus must include an alias-backed single child, alias-backed vector, nested-option
absent/present behavior, qualified `std::option::Option`, unexpected annotated type (compile
failure), undeclared slot, wrong kind, same id under a second slot, duplicate non-`many`, a valid
`many` collection, missing/extra wire child, full-dialect mismatch, parent mismatch, and a
zero-publication assertion on each rejection. Its language-neutral oracle reads only the declared
slot rows and borrowed-field fixture values; it must not call the Rust projection.

Do not layer the new projection over `store::ArtifactRefs::child_refs`: its default is an empty
vector ([`store:2832`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2832)) and its
only implementations are fixtures. That is precisely the false-empty authority this packet must
eliminate. The new schema visitor supersedes its child half; any independent link reader remains
outside this admission packet until it has a separately proven restore consumer.

### Current single-child transaction check

The current `admit_child_member` split does not introduce a pre-open publication leak. Its
`ChildContentView::admit_member` is only a pure index/hash lookup
([`plugin:8491`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8491)); the
content-generation call is capacity validation, and the graph ticket is read-only until commit
([`store:18699`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18699)). The
member-map ticket is acquired last at [`plugin:19902`](../../../../../../🧰️framework/🛍️products/💻️os/🔌️plugin/🦀️.rs:19902).
Accordingly graph or map admission failure cannot leave a root candidate to retire; the existing
`ChildMemberRetirement` begins at the only fallible point after an owned member exists, failed
post-open preparation. This classification is source-only and does not prove its registered
native laws.

`MemberFactory::open` deliberately has no cancellation parameter
([`store:17806`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17806)). A
future dropped while opening must therefore rely on its own owned-input/drop discipline. Do not
retrofit a synthetic cancellation path into this single-open transaction; the forthcoming tag-14
batch loader needs its own retained page/member ticket and bounded cancel/abort lifecycle.

## Current Schema Projection Review — Source-Only RED

The current tree now contains the proposed visitor and borrowed projection. `ChildFieldRefs`
correctly gives `Option<T>` its inner cardinality and `Vec<T>` `MANY = true`, charging a visitor
step for absent and container values at
[`schema/component.rs:151`](../../../../../../🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:151). The derive
uses every explicit `#[child(kind = "…")]` annotation as authority rather than guessing the
spelling of the field type ([`schema derive:226`](../../../../../../🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs:226)); aliases and nested `Option`/`Vec`
therefore receive a real trait-bound error if they are not child fields. `ChildRestoreProjection`
uses a fixed 64-reference/256-step/256-byte budget and rejects invalid static slots, wrong kind,
singular duplication, same-id cross-slot duplication, and incomplete exact sets before an
admission ticket can be committed ([`store:2803`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2803)). This is sound as a
*projection primitive*; its 15-row neutral corpus plus the separate derived-alias boundary law
are useful source evidence, not a public restore proof.

The first real build (coordinator session `62694`) is RED: the initial placement makes the kernel
depend on full `semio-framework-schema` while full schema already depends on kernel for state and
catalog types. This is an architectural, not taxonomy, blocker. The clean repair is to place only
`ChildSlotSpec`, `ChildRefFields`, `ChildRefVisitor`, and `ChildFieldRefs` in a pure kernel
`os_schema_composition` module, re-export them once from the full schema crate, and have store use
the kernel module. The derive integration test can remain in schema (which legitimately depends
on kernel); the neutral projection corpus remains kernel-only. Do not add a new crate, optional
dev edge, or a kernel-to-full-schema dependency. No native proof has passed.

Even after that compile repair, the feature is **not yet an admission authority**. Both public
methods still accept raw `(slot, child_id, dialect)` and call `admit_child_member` directly:
`open_child` at [`plugin:19957`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19957) calls `M::open` at 19963, and
`register_child` at [`plugin:20000`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20000) admits an already-built member.
Neither reads the live parent snapshot or calls `ChildRestoreProjection`; an arbitrary slot/id
therefore still reaches the pre-open admission sequence. Existing positive restore tests prove
the bypass explicitly: `TestSnapshot` has no `ArtifactChild` field, yet the test invokes
`restored.open_child("slot", "child-1", …)` at
[`plugin:34904`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34904). They must not be credited as composed restore
acceptance.

The minimal fail-closed wiring is a default-`Err` app hook taking the **loaded** snapshot and the
candidate member identity, for example an `ArtifactApp::authorize_child_restore(snapshot, slot,
expected) -> Result<(), Fault>`; `EditorApp` and `ViewerApp` forward it. A composed app derives a
projection and calls `admits_member`, while a leaf app denies rather than inheriting an empty
projection. `VcsArtifactApp::open_child` builds the expected child ref and exact owner from its
own live envelope, invokes that hook *before* `admit_child_member`/`M::open`, and retains its
existing map/graph/root transaction. The batch path must use `admit_complete`, not repeatedly
call the single hook. The `ArtifactDialect` carrier deliberately has free `standard`/`subset`
strings ([`io schema:68`](../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:68)); the admission
guarantee is equality to the loaded parent reference plus the closed `space_members!` full
coordinate match, not an invented standard grammar.

`register_child` is a distinct genesis boundary and must not remain a public raw bypass. Its only
non-test caller is Flow's test/app helper, which obtains a genuine `FlowSnapshot.content` but
then directly invokes `register_child` at
[`Flow editor:2058`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2058). It should either consume a
private parent-mutation/`ChildGenesis` receipt, or (for reloading that already-declared snapshot)
be replaced by `open_child`. No other production invocation currently exists. That makes the
future packet precise: raw `register_child(slot,id,dialect,member)` becomes private receipt
commit; persisted reload enters only `open_child`.

### Genuine Public Fixture

Use the existing Flow test app, not a `TestSnapshot.label → child` map and not a hand-written
`ArtifactCompositionFields` implementation. `FlowSnapshot` has a persisted,
derive-annotated `content: FlowContentChild` at
[`Flow snapshot:31`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:31), and
the existing `FlowApp = VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>` setup is already
the real public app/member path ([`Flow editor:2051`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2051)).
Build one real content child envelope whose owner is the live Flow parent ref and whose full
coordinate equals `snapshot.content.target`; call public `open_child` with that pack. Assert one
member-map entry, exact owner, graph edge, typed root and one generation. Then use the same valid
pack for wrong slot, different child id, wrong full dialect, and wrong parent-owner rows; each
must fail before factory open and leave all five observations unchanged. A tiny test-only
`MemberFactory` counter wrapping the actual `SemioMembers` open arm is acceptable solely to prove
the pre-`M::open` ordering; it must delegate the success arm to the real factory and must not map
labels or manufacture child identities. Every success/rejection must close the Flow app/member
with the existing bounded retirement loop. The neutral JSON oracle remains independent and is
extended with the owner/full-dialect rows; it must not call Rust projection code.

## Current Kernel-Law Compile Frontier — Non-SPR Test Boundary Review

### Registered target remains intact

The registered `member-dialect-check` target at
[`os rust script:65`](../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:65)
still selects the exact twelve assertions: four schema laws, the five private kernel member
laws, and three plugin laws. Its runner compiles each package's complete `--lib` test binary
before selecting exact FQNs. The historical `79256` attempt reached the kernel binary build
after the four schema laws and stopped at test compilation; it did **not** execute any of the
five kernel assertions. This is a compile frontier, not a failed member-admission assertion.

The five kernel laws must remain in that target. They are the only focused checks of closed
dialect/factory identity, graph admission, graph replacement, and parent projection. Moving
them to an integration crate, filtering them out, or weakening the binary-wide preflight would
make the registered gate vacuous. The repair scope is therefore only the test-only dependencies
and fixture/oracle code needed to compile the current kernel `--lib` test binary.

### Superseded historical diagnostics

The first error groups in `🗑️generated/member-dialect-exact/exact-cargo-laws-pKuwf4/01/build.stdout`
are no longer current bytes: `LossyDiff` is now `ToValue`/`FromValue` only
([`store:21800`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21800)); the
lossy fixture imports the first-party derives; `MiniDoc`/`MiniDiff` use qualified derives; and
the directory test has a qualified `VecDeque`. Treating those earlier E0277/E0412 messages as
new production or member-factory failures would be stale attribution. The SPR protocol fixture
and DSL ambiguity groups are separately owned; this review deliberately does not recast them as
kernel-admission defects.

The formerly direct serde calls on first-party-only values are also source-closed in the current
tree. `HistoryColumn` now passes its `DslValue` through the test-only
[`SerdeValue`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19488) projection
at [`store:24296`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:24296), and
the metadata/message/conflict byte checks do the same at
[`store:25397`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:25397). No
production `Serialize` derive was restored on those domain types.

### Serializer-oracle ordering rule

`SerdeValue` is the correct narrowly-scoped differential encoder: it walks the ordered
`DslValue::Object(Vec<(String, DslValue)>)` entries with `SerializeMap` in stored order
([`store:19491`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19491)). Thus
`serde_json::to_string(SerdeValue(&value.to_value()))` is a valid independent byte encoder for
a claim about first-party JSON ordering. It neither requires nor grants serde to the production
type.

By contrast, the general `DslValue -> serde_json::Value` bridge collects an object into a
`serde_json::Map` ([`value:218`](../../../../../../🧰️framework/🔨️modules/🌱️value/🦀️.rs:218)). It is
appropriate for fixture parsing, hostile-object mutation, and structural observations, but is
not a canonical-byte/order oracle. When a test intentionally discards order it should make that
choice explicit with `pack::json::value_eq_ignoring_object_order`
([`pack json:517`](../../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:517)); arrays remain
order-sensitive. In particular, `assert_fixture_case`'s JSON bridge at
[`store:20098`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:20098) is a
valid structural third-party decoder exercise, but must not be renamed or expanded into a
canonical JSON byte assertion. Such an assertion must compare
`os_pack::json::to_json_string` directly with fixture bytes and, if differential evidence is
needed, use `SerdeValue`, not a re-stringified `serde_json::Value`.

### Compound-envelope capture/read review

The production boundary is correctly singular: `ArtifactStore::envelope_json` calls
`ArtifactEnvelopeOwners::capture_read` before invoking the first-party JSON writer
([`store:15933`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15933)).
`capture_read` first rejects mismatched history/cursor visibility authorities, captures the
shared decision once, and supplies that same decision to both borrowed readers
([`store:2362`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2362)). There
is no serializer-side fresh-reader bypass in the production method.

The current neutral `group-read.json` law is now source-consistent with that boundary. Its
`GroupReadTriggerSnapshot: ToValue` commits only while the already-captured read is converted
([`store:22928`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22928)); the
captured history/cursor are serialized before a fresh capture is made at
[`store:22984`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22984). Its
fixture rows therefore distinguish pending, commit-after-capture, pre-committed, and aborted
decisions without reintroducing serde on `ArtifactEnvelope` or its borrow-only read view. The
foreign-authority case directly asserts `capture_read()` failure before its conversion branch
([`store:23048`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:23048)). This
is a **source-only PASS** for the capture ordering, pending the retained native gate.

One small test-strengthening opportunity remains, not a production defect: add a conversion
counter to the foreign-authority trigger and assert zero after the rejected `capture_read`. The
current negative fixture has no trigger (`commit: None`), so it proves the error return but
cannot independently observe a hypothetical conversion side effect. The required property is
strict: foreign authority must fail before *any* `P::to_value`; no new serializer for
`ArtifactEnvelope` is warranted.

### Bounded next action

Repair only remaining live non-SPR test-only edges by translating any direct serde assertion on a
`ToValue`/`FromValue` domain type to either (a) `SerdeValue(&type.to_value())` for exact ordered
byte parity, or (b) `serde_json::Value` after first-party encoding for structural/hostile
fixtures. Never add test-only serde derives to a type merely to quiet a transitive bound (notably
`ArtifactRef`-bearing child/link structures). Re-run the unchanged twelve-law registered command
only after its owners finish the binary compilation repair; this audit ran no Cargo command and
makes no runtime-pass claim.

### Re-read After the Current Serde Boundary Repair

The current correction removes the invalid serde path from `ArtifactChild` and the
`SpaceCheckpoint`/`CommitSpaceCheckpoint` chain. That is the right direction: an
`ArtifactChild` contains an `ArtifactRef` and local-owner semantics that must not acquire a
test-only wire contract merely to compile a test. The remaining `SpaceMemberPin` and
`SpaceAlternative` test-only derives contain scalar-only fields and are not transitive evidence
of a checkpoint serde obligation. The existing `SerdeValue` parity cases retain their assertions;
they prove *writer fidelity after `ToValue`*, not an independent semantic encoder of each domain
type. Keep neutral expected JSON/text fixtures for semantic field/absence/enum coverage rather
than presenting `SerdeValue` equality alone as a full schema oracle.

## Flow Public Parent Fixture — Current Source RED

The newly strict projection makes the existing Flow test helper an honest, useful production
fixture only after two real identity corrections. `FlowSnapshot.content` declares
`#[child(kind = "s.stdio.semio.flow")]` at
[`Flow snapshot:29`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:29),
but its content factory creates a target with `artifact_kind = "s.stdio.semio"`, standard `v1`,
subset `flow`, and literal `artifact_id = "flow-content"` at
[`Flow content:187`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:187). The
same factory gives the child a content-addressed `child_id` beginning
`flow-content-sha256-…` ([`Flow content:186`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:186)).

`ChildRestoreProjection` intentionally requires (1) exact `artifact_kind == ChildSlotSpec.kind`
and (2) `child_id == artifact_id` before it accepts a row
([`store:2884`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2884)). Thus a
real Flow snapshot currently fails projection twice. This is not a fixture quirk: it is the
precise protection needed to prevent an arbitrary target identity from being admitted under a
declared slot.

The minimal source-owned correction is to state `kind = "s.stdio.semio"` (the artifact-kind
coordinate, not a dotted reconstruction of its subset) and make the child target's
`artifact_id` equal the already-content-addressed `child_id`. The exact full dialect remains
`s.stdio.semio@v1/flow`, and is independently closed by the existing `SemioMembers::Flow` arm
([`stdio members:1341`](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1341)). Do not
relax the projection equality or add a name-to-id exception.

There is a second, immediately observable test-compilation consequence of the correct
`ArtifactChild` serde removal: `FlowSnapshot` still requests a test-only serde derive at
[`Flow snapshot:18`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:18),
and Flow's local-owner test directly invokes serde on `FlowContentChild` at
[`Flow content:294`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:294). Neither may
restore serde on the child handle. Replace only that test oracle with first-party
`ToValue`/`FromValue` plus a third-party parse of the resulting JSON for the two public fields;
assert the decoded child has no local owner. This preserves the local-only non-leak assertion
without inventing a wire contract.

After those identity and oracle repairs, replace `register_content_child`'s raw
`register_child("content", …)` call at
[`Flow editor:2052`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2052)
with the public persisted-child `open_child` path and a real envelope pack. The positive law
then uses a loaded Flow parent plus the exact child target/owner; hostile rows vary slot,
child-id, parent owner, and one full-dialect field. Each rejection must show no factory call,
no child map/graph publication, and no generation change, followed by bounded app/member close.
Until the app boundary actually calls `ChildRestoreProjection` before `admit_child_member`, this
remains a source-design requirement rather than a claimed public restore pass.

## 2026-09-04 Current Preflight and Retained-Rejection Frontier

### HistoryLog identity preflight — source PASS, bounded claim only

The live `open_member_store` now decodes the outer pack and `HistoryLog`, then calls
`validate_member_history_identity` before `P::decode_pack`, document replay, or
`ArtifactStore::new` ([`store:3114`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3114)).
The predicate compares the persisted document id and schema, requires the full persisted
composition dialect, and parses the persisted parent URI before comparing the exact parent,
slot, and child id ([`store:3130`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3130)).
It therefore closes the observed wrong-document/wrong-owner terminal-envelope route without
trusting a stringly parent equality. A caller-supplied owner with an inconsistent child id is
also denied. This is deliberately not an assertion that raw history parsing is avoided: it is an
assertion that no typed snapshot/store construction begins before the authoritative identity
decision.

The new test-only `MEMBER_SNAPSHOT_DECODE_COUNT` is placed at the real
`DemoSnapshot::decode_pack_with` boundary ([`store:21657`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21657),
[`store:21668`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21668)). The
neutral admission corpus resets and reads it before any later oracle decode, expecting exactly
zero for rejected identity rows and one for accepted rows ([`store:21993`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21993)); malformed outer packets also
assert zero ([`store:22023`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22023)).
That is a truthful, test-local single-decode law. It does not count `HistoryLog` or mutation
decoding, and must not be presented as a general “no parsing” or cancellation proof. A future
matching-header malformed-record case must instead prove that every typed value already made is
retained and closes under fixed grants.

### Remaining typed-owner loss — RED

The preflight does not eliminate later fallible ownership transitions. `parse_decoded_document_spr`
can build its terminal `ArtifactEnvelope` and subsequently fail in composition application,
durable-history validation, or cursor replay ([`store:10930`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10930)).
Likewise `ArtifactStore::new` validates/folds/seeds after accepting a terminal envelope
([`store:13885`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13885)). A `?` in
either sequence may drop an envelope whose `Drop` contract requires explicit detachment.

The smallest sound packet is not a generic close loop:

1. Retain `ArtifactEnvelopeOwners` while applying history composition, validating durable
   history, and replaying the cursor. Construct the terminal `ArtifactEnvelope` only after these
   fallible stages, or add owner-borrowing variants of those validators. This immediately removes
   the malformed-history terminal-shell path without changing the acceptance contract.
2. Before the first typed `P` or `Mutation` decode, obtain the exact
   `MemberStoreOwners<P, Mutation>` factories and change the member-open result to a closed,
   retained error such as `MemberStoreOpenRejected<P, Mutation> { Empty | Parse(..) |
   Initialization(..) }`. `Empty` is valid only before any terminal/typed owner exists; the other
   states implement a one-step, grant-bounded close.
3. Build the initialization state around the existing
   `ArtifactStoreInitializationRuntime<P>` ([`store:12276`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12276)) and delegate the remaining
   envelope to `ArtifactStoreEnvelopeRetirement` ([`store:1250`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1250)). The former already has a bounded
   `close_step`; the latter is the existing terminal-envelope closer. Do **not** reuse
   `ArtifactEnvelopeDecodeAuthority` (it owns an unrelated schema-record cursor) or
   `ArtifactStoreResolutionCandidateRetirement` (it assumes a fully live store and contains the
   prohibited generic long close loop).
4. In `VcsArtifactApp::open_child`, reserve its existing fixed
   `child_admission_abort_retirements` slot before invoking `M::open`; on a nonterminal factory
   rejection, cancel map admission and insert the retained rejection into that same registry.
   The generated `space_members!` enum must carry one closed rejection arm per concrete member
   factory. This keeps failure cleanup in the existing maintenance/close authority and prevents a
   successful-looking map/graph publication.

Required new native rows: matching header plus corrupt history composition; matching header plus
malformed mutation/replay; `ArtifactStore::new` failure after an initialized current snapshot;
and a one-item/4096-byte repeated close of each retained error to terminal. Every row must assert
no child map, graph edge, root, or generation publication. A cancellation row must drop the
factory future after its first typed decode and prove the same bounded retirement path.

### `root16799` compile diagnostic — current RED, deduplicated

Coordinator evidence reports that the preceding two diagnostic groups passed nine source laws;
this agent did not run Cargo. `root16799` then failed while compiling the plugin `--lib` binary,
with 1,260 emitted diagnostics recorded in
`🗑️generated/member-dialect-exact/exact-cargo-laws-PqzIuu/02/build.stdout`. They are not 1,260
independent defects. The primary current families are:

- Missing first-party derive imports in the seven mutation fixture roots cause the vast majority:
  `TestMutation` (749 `ToValue`/`FromValue` cascades), `TxnMutation` (96),
  `TestConfigMutation` (76), `DummyMutation` (60), `SurfaceMutation` (46), and the publication
  fixtures (82). Restore the existing `semio_framework_value_derive::{ToValue, FromValue}`
  imports at the fixture roots; do not add serde bounds to production mutation traits.
- Five identical stale `include_str!` paths in
  `🕹️interaction/📡️live/📨️dispatch/🧪️tests/🦀️.rs:33,49,61,80,102` still name
  `.../🎭️actor/🚪️lifetime/🧪️fault.fixture.json`, while the live physical fixture is
  `🚨️fault.fixture.json`. This is a test-path correction, not a lifetime protocol failure.
- `ChildrenTestDiff` is a unit struct with `ToValue`/`FromValue` derives at
  `🔌️plugin/🦀️.rs:37745`, which the derive deliberately rejects. Make the test diff a named-field
  empty struct or hand-write the first-party value traits; do not change derive behavior.
- The shared declaration-channel law at
  `🧪️tests/🛰️declaration-channels/🧪️tests/🦀️.rs:12,56-140` asks serde to encode
  `MutationLeafDescriptor`, generic `M`, and `M::Diff`. Those are intentionally first-party
  `ToValue`/`FromValue` values. Use `protocol::json`/`dsl::os_pack::json` for the subject and use
  `serde_json` only as the independent parser of the resulting JSON, as the existing SPR mutation
  laws do. Do not impose `Serialize`/`DeserializeOwned` on `Mutation`.
- Inference helper tests use serde directly for the same first-party wire structs at
  `🏗️builder/🦀️.rs:1053` and `⚛️reactor/💼️jobs/💡️infer/🦀️.rs:422,447`, despite the production
  boundary already using `protocol::json` at `🔌️plugin/🦀️.rs:1816,1826,1895`. Align the tests with
  that boundary rather than deriving a second serializer.
- The new member-admission white-box law is a sibling of `component::app` and directly reads
  private fields at `🔌️plugin/🦀️.rs:34840-34905`. Relocate that test module within
  `component::app` so it can retain its exact registry/generation assertions. Widening the
  production fields, or weakening the test to merely observe `close_step`, would hide the
  retained-failure property it is meant to prove.

Further independent test-only roots after those are: add the new `mutation_payload_facet` field
to two explicit `MutationLeafSourceScope` literals (the contributed-wire leaf and job-test
fixture); replace `ArtifactKindId`'s private reexport import in the artifact-admission test; and
keep `serde_json::Value` out of `ToValue`/`FromValue` generic calls. These are bounded API/test
drift corrections. They neither invalidate the twelve-law target nor justify skipping its full
test-binary preflight.

### Follow-up: test-only white-box observation — source design PASS

The initially suggested test relocation is superseded by a smaller, better-scoped solution. Keep
the existing exact FQN at `🔌️plugin/🦀️.rs:34834`; add only a
`#[cfg(test)] pub(super)` scalar observer inside the top-level `app` module. It should return
primitive observations (or a test-only scalar struct) for: child-map emptiness,
child-content-root emptiness, child-content generation, failed-admission-retirement emptiness,
and failed-admission generation. It must not return a member, root, registry, or retirement
handle.

That lets the law retain the important sequence: malformed input leaves all publication state
empty at generation zero; a post-open pin failure leaves precisely one pending failed-admission
owner at generation one; and production `maintenance_step` stage 20 drains it to empty. The law
must **not** call the private `child_admission_abort_step` directly. This is a test-only read
surface, not an exported runtime API, and preserves the actual maintenance close funnel at
[`app:19941`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19941) and
[`app:24077`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24077).

### Next test-codec corrections — source map

The remaining declaration/wire/value diagnostics all share one rule: the runtime contracts use
first-party `ToValue`/`FromValue`, while `serde_json` is only an independent parser of canonical
text.

- In `🧪️tests/🛰️declaration-channels/🧪️tests/🦀️.rs`, replace the generic
  `Serialize`/`DeserializeOwned` requirements on `S`, `M`, `M::Diff`, and `L` with the
  first-party value traits. Decode the JSON fixture through
  `protocol::json::from_json_str(&value.to_string())`, encode through
  `protocol::json::to_json_string`, and then use `serde_json::from_str::<serde_json::Value>`
  only to compare the produced text with the neutral JSON object. This retains the independent
  parser and avoids making `Mutation`, `MutationDiff`, or `MutationLeafDescriptor` serde APIs.
- `Plugin::wire_list_artifact_inference_services` is already produced with `protocol::json`
  ([`plugin:1816`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1816)); the
  builder roster test and infer-job request/result fixtures must use the same codec rather than
  `serde_json::{to_vec,from_slice}`. The source wire request/result types intentionally derive
  only `ToValue`/`FromValue` ([`plugin:1556`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1556)).
- Opening relay test `replay` currently asks generic `from_dsl_value` to produce a
  `serde_json::Value` ([`plugin:30573`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30573)).
  Convert the existing `DslValue` directly via its established `serde_json::Value` bridge; do
  not invent `FromValue for serde_json::Value`.
- The interaction capture/query tests encode/decode `LocalInteractionIdentity` and
  `LocalInteractionCapture` through serde at `🕹️interaction/📖️capture/🧪️tests/🦀️.rs:19`,
  `🕹️interaction/📡️live/🧪️tests/🦀️.rs:98`, and
  `🕹️interaction/📡️live/📨️dispatch/🧪️tests/🦀️.rs:386`. Build/decode the subject through
  `protocol::json`, then feed the resulting canonical text to serde only for fixture-shape
  comparison. This preserves the large-page/ACK/retirement law rather than adding serde to the
  local interaction authority types.

These are current source recommendations only; no recompilation or runtime assertion was run by
this audit.

### Exact member-initializer shape

`MemberStoreOwners<P, Mutation>` already owns the non-cloneable store disposer and the cloneable
initial-snapshot/mutation retirement factory arcs ([`store:1971`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1971)). Consequently the retained path must take the
owners *by value* before typed construction; reconstructing them after an error would incorrectly
invent a second disposal authority. The narrow target signature is:

```rust
pub async fn try_new_member(
    envelope: ArtifactEnvelope<P, Mutation>,
    owners: MemberStoreOwners<P, Mutation>,
) -> Result<ArtifactStore<P, Mutation>, MemberStoreOpenRejected<P, Mutation>>;
```

Keep the broad `ArtifactStore::new` for callers that do not have member ownership, and make
`open_member_store` call this member-specific constructor. On success it consumes `owners` via
the existing `from_initialized_runtime_with_owners` path; it must not construct a plain store and
install owners afterward. On failure, `MemberStoreOpenRejected` owns `Option<MemberStoreOwners>`
and either an envelope retirement, an initialization runtime plus envelope retirement, or the
earlier parser-retirement state. It borrows/clones the two factory arcs only to create the already
existing bounded retirees. This is sufficient to preserve the one real disposer while allowing
each `close_step(1, bytes)` to release only one existing sub-owner.

The existing `ArtifactStoreInitializationRuntime<P>` is exactly the post-fold retained carrier:
it already tears down current snapshot, ids, cursor, causal graph, and revision state one step at
a time ([`store:12276`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12276)).
`ArtifactStoreEnvelopeRetirement` is the matching terminal envelope carrier
([`store:1258`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1258)). Neither
`ArtifactEnvelopeDecodeOwnerBundle` nor `ArtifactStoreResolutionCandidateRetirement` has this
ownership shape. The former is for a different field-decoder protocol; the latter starts from an
already-live store and is not a permissible open failure recovery mechanism.

## 2026-09-04 Interaction Byte Oracle and Public Parent Projection

### Capture/query byte oracle — source PASS

The two actual Store-backed fixture constructors no longer ask serde to serialize
`LocalInteractionIdentity`. They build the independent JSON object from primitives: the instance
is numeric, the generation is its decimal string, and each revision is formatted as lowercase
two-digit bytes ([`capture test:20-29`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🧪️tests/🦀️.rs:20),
[`query test:21-30`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/🦀️.rs:21)).
This precisely matches the producer's fixed canonical JSON identity: `CapturedRoot` emits
`appInstanceId`, `documentRevision`, `generation`, `revision`, and `topologyRevision` as the
corresponding primitive/hex forms ([`capture producer:26-48`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:26)).

That keeps `serde_json` on the only legitimate independent-oracle side: it assembles and prints a
neutral JSON tree whose bytes are compared against the first-party canonical cursor at grants 1,
64, and 4096. No core `ToValue` implementation, no production capture type, and in particular no
`LocalInteractionIdentity`, regains a serde requirement. The first-party protocol remains the
typed decode authority for the live large-page tests
([`live query test:98`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🧪️tests/🦀️.rs:98),
[`dispatch test:386`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/🦀️.rs:386)).

### Composed-parent admission — current source RED, staged negative not executed

`ChildRestoreProjection` now provides the right borrowed predicate:
`from_snapshot` walks derived child fields under fixed reference/field/step limits, and
`admits_member` checks the exact slot, child id, artifact id, kind, standard, and subset
([`store:2844-2873`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2844)). The
new test fixture is a genuine derived parent, not a label-to-child substitute:
`ComposedParentSnapshot.slot` is an `Option<ArtifactChild<TestSnapshot>>` annotated with
`#[child(kind = "s.test.child")]`
([`composed fixture:2-10`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:2)).
Its `HAS_CHILD = false` negative uses a correctly parent-owned, persisted child envelope and
asserts no child-content generation publication ([`composed fixture:90-104`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:90)).

It is intentionally RED in the current source. No `ArtifactApp`, `ArtifactEditor`, or
`ArtifactViewer` projection hook exists, and `VcsArtifactApp::open_child` immediately calls
`admit_child_member` ([`app:19981-19987`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19981)).
That admission reserves a child-content root, a next generation, a graph reservation, and a map
admission before `M::open` ([`app:19901-19915`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19901)). Thus the current fixture is source-backed but has not demonstrated the desired denial; do
not call it a passing law until it is registered and its observed RED is captured.

The smallest closed hook is a static method on the author-facing traits and runtime trait,
forwarded by `EditorApp<E>` and `ViewerApp<V>`:

```rust
fn child_restore_projection(
    snapshot: &Self::Snapshot,
) -> Result<store::ChildRestoreProjection<'_>, Fault>
```

Its default must return an explicit denial, never an empty projection. A composed app calls
`ChildRestoreProjection::from_snapshot(snapshot)`; an app with no declared field receives no
restore authority. In `VcsArtifactApp::open_child`, materialize the current parent snapshot,
call that hook, and require `projection.admits_member(&slot, &expected)` **before**
`admit_child_member`. This keeps an unlisted/cross-slot/wrong-dialect child out of every
reservation, graph, map, factory, and retirement path. It does not widen `ArtifactApp::Snapshot`
with `ArtifactCompositionFields`, which would break every leaf app; it gives only composed owners
the schema-derived authority.

`PluginApp::load_child_pack` is separately incomplete for an exact full restoration set: its
current `AppCommand::LoadChildren` handler loops each entry and invokes `load_child_pack` one at a
time ([`app:31379-31385`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31379)),
while `ChildRestoreProjection::admit_complete` is expressly a whole-set predicate
([`store:2875-2887`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2875)). The
per-child hook is a truthful P0 gate. A later retained `load_children` batch must validate all
entries with `admit_complete` before any member open/publication; do not pretend that one-at-a-time
loading proves omission or duplicate rejection.

There is only one production caller of this restore path: `PluginApp::load_child_pack` delegates
to `open_child` ([`app:24594-24597`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24594)).
The remaining direct opens are private `TestApp` laws. Accordingly a fail-closed default must make
the old `TestApp` positive restore at [`app:34930`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34930)
fail: `TestSnapshot` has no declared child field. Move that success assertion to
`ComposedParentApp<true>` and its real derived `slot`; do not install an empty/TestApp acceptance
hook. Direct `register_child` remains the distinct explicit genesis/transfer operation and must
not be used to bypass persisted-child restore admission.

### Required post-open parent fence

`M::open` is asynchronous. Although the projected parent is checked before starting it, the
commit must not rely on that older observation. Capture the parent's current store generation with
the initial projection, then after a successful `M::open` and before `prepare_child_member` or
`commit_child_member`, obtain a fresh parent snapshot, require the same generation, and rerun the
author hook plus `admits_member`. Rechecking both is intentional: a generation comparison is a
freshness guard, not the schema-derived membership decision.

If that second check rejects, first cancel the map admission and then move the already-open member
into the preflighted `child_admission_abort_retirements` slot exactly as the existing
post-preparation failure path does. The existing `OwnsAdmission` needs no artificial cancellation:
`CompositionGraph::admit_owns` is read-only until `commit_owns_admitted`
([`store:18845-18865`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18845)), and
the native graph corpus explicitly checks that an admission does not alter the graph
([`store:26620-26625`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26620)).
Dropping the uncommitted ticket is therefore the correct nonpublication path.

The focused hostile law needs an `M::open` fixture seam that changes/removes the real parent's
declared child field while the open future is suspended. Its postcondition is: no child map, graph
edge, child-content root, or generation publication; exactly one abort-retirement owner; and that
owner reaches terminal empty only through the production stage-20 maintenance funnel under
one-item/4096-byte grants. This is a source requirement pending the registered execution, not a
claimed race pass.

### Hook and fence landed — source PASS, race nonclaim retained

The hook is now present with an explicit denial default on `ArtifactApp`
([`app:11011-11017`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11011)),
and matching denial/forwarding on `ArtifactEditor`, `ArtifactViewer`, `EditorApp`, and
`ViewerApp` ([`app:26034-26041`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26034),
[`app:26609-26613`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26609),
[`app:26877-26880`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26877)).
`ComposedParentApp` supplies the only positive fixture projection from its actual derived
snapshot ([`composed fixture:49-55`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:49)).

`open_child` now constructs the expected full coordinate, validates the loaded-parent projection
before any admission, records the parent generation, and after `M::open` requires both an
unchanged generation and a newly evaluated projection before preparation/commit
([`app:19995-20030`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19995)).
The stale/error branch cancels the map admission and transfers the opened member to the existing
bounded failed-admission retirement registry. This is the correct P0 source shape.

There is an important nonclaim: the current operation owns `&mut self` across `M::open`, and the
member factory receives only expected identity, owner, and bytes—not a mutable parent
capability. Therefore safe current code cannot produce a true concurrent parent mutation during
that await. A test that injects one through global/interior test machinery would not be a
production-derived race proof. The generation/projection fence is useful future-proofing for a
retained operation that may release exclusive app ownership; an authentic interleaving law belongs
with that future lifecycle. The current neutral parent corpus is an admission/authority law, not a
claimed concurrent-race pass.

### Coordinator-observed gate frontier

Coordinator session `10115` compiled the complete plugin `--lib` binary (539 discovered tests),
with the four schema and five kernel laws green. The first plugin member law reached its runtime
assertions and debug marker, then failed in a standalone fixture envelope `Drop`; the reported
cause is fixture ownership, not a production admission assertion. This audit did not run Cargo.
The next registered selector is expected to execute the composed-parent negative before the hook
is added, providing the required observed RED baseline.

## 2026-09-04 Flow content-child identity and real `SemioMembers` restore P0

### Current verdict: RED — one child is described by incompatible full coordinates

The generic parent gate is now source-closed, but Flow cannot use it yet. The production child
handle names a content address as its `child_id`, then gives its target the unrelated static
`artifact_id: "flow-content"` ([`flow artifact:186-190`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:186)). The new projection intentionally requires those ids to be
identical, as well as kind/standard/subset equality. This is a real restore denial, not a
formatter difference: Flow's `SemioMembers::Flow` arm is bound to the three-part dialect
`s.stdio.semio@v1/flow` ([`stdio members:1107-1128`](../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1107)).

The same incompatible coordinate is emitted by every current production identity producer:

1. `flow_content_child_from_digest` makes `flow-content-sha256-<digest>` but targets
   `flow-content` ([`flow artifact:186-190`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:186)). This one helper feeds default snapshots,
   mutation diffs, direct editor materialisation, and the bounded public content builder
   ([`flow artifact:164-180`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:164),
   [`flow editor:646`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:646)).
2. The retained preparation writer independently pages the digest into field 3 but pages the
   literal `"flow-content"` into field 4, then constructs its `ArtifactRef` from those two
   fields ([`preparation:137-170`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:137)). It must derive both fields from the one complete paged
   spelling (`3 | 4`), rather than give the target a second identity source.
3. Both derived Flow parents still declare four segments, `s.stdio.semio.flow`, rather than the
   child artifact kind `s.stdio.semio`: `FlowSnapshot.content`
   ([`snapshot:23-31`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:23)) and the aggregate
   `FlowArtifact.content` ([`schema:77-92`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:77)). The subset remains
   `flow`; it is not appended to `artifact_kind`.
4. Neither `FlowPlayApp` nor `FlowViewer` overrides its now fail-closed
   `child_restore_projection` trait method. The framework forwarding is real
   ([`plugin app:26609-26613`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26609),
   [`plugin app:26877-26880`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26877)); the omission is therefore an explicit error at the concrete Flow
   editor/viewer boundary, not an inferred empty list. The editor needs
   `ChildRestoreProjection::from_snapshot(snapshot)` mapped to a `Fault`. The viewer should
   implement the same read-only projection for its real `FlowSnapshot`, even though
   `FlowApps::FlowViewer` deliberately uses `NoMembers` and so has no child-opening authority
   ([`flow plugin:10-14`](../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️.rs:10)). A projection does not grant
   a factory or write capability; it only truthfully describes its loaded parent.

The source test is correctly TDD-red at this revision. Its independent Node/AJV oracle already
requires the exact target identity, canonical kind, and retained `3 | 4` digest paging
([`fixtures script:229-261`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts:229)); current source still contains all three rejected
spellings. Its neutral corpus fixes the intended coordinate without treating a generated row as
authority: `childSlot: content`, `targetIdentity: child-id`, and
`s.stdio.semio/v1/flow` ([`content identity fixture:1-8`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🪪️content-identity/🔣️.json:1)),
with a strict AJV schema ([`fixture schema:1-35`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🪪️content-identity/🧬️.schema.json:1)).

### Atomic correction census

The production four-item correction is: make the helper's target id `child_id.clone()`; make
retained fields 3 and 4 both stream that digest spelling; change both `#[child]` kinds to
`s.stdio.semio`; and add the editor/viewer derived projection overrides. No alias,
`s.stdio.semio.flow` compatibility acceptance, or static target-id fallback is admissible.

Then atomically repair the fixtures so all persisted shapes obey the same invariant:

- 20 Flow mutation before/after JSON snapshots currently carry a fixture `childId` but static
  `target.artifactId: "flow-content"`; each target id must equal its existing fixture child id.
- The checked-in demo DSL has the same mismatch at
  [`demo asset:10-17`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio:10).
- The local owner ABA fixture in `flow artifact` builds
  `child_id: "flow-content-reused"` against the static target at
  [`flow artifact:253-260`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:253); repair it
  too, so the test cannot normalize a malformed persisted coordinate.

Fixture ids need not be recomputed as content hashes merely to test mutation semantics; the
universal restore invariant is equality between `childId` and `target.artifactId`. The five-row
content-identity corpus separately proves the real production SHA-256 naming and its
domain-separated bytes at grants 1, 64, and 4096. This keeps the fixture conversion bounded and
does not quietly alter the historical mutation oracle.

### Smallest honest public restore law

Extend the existing Flow editor testkit rather than inventing a synthetic member. It already
creates the actual `SemioMembers` union through `create_semio_member`, encodes the real
`SemioFlowSnapshot`, and adopts it under the concrete `content` slot
([`flow testkit:2052-2059`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2052)). The law should:

1. Build a source `FlowApp` through that testkit; borrow its real Flow member only long enough to
   obtain `envelope_pack_bytes`, then release the borrow.
2. Close the source app through `PluginApp::close_step(1, 4096)` until
   `close_terminal_is_empty`; do not drop an owned source member or app. The generic helper at
   [`plugin testkit:6727-6738`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6727) is
   `NoMembers`-only and therefore cannot be reused for this test. Add the equivalent bounded
   Flow-local test helper with the concrete `SemioMembers` parameter.
3. Construct a second real `FlowApp` **without** calling `register_content_child`; read its loaded
   parent `content` handle, then call public `open_child("content", exact_child_id,
   exact_dialect, envelope_pack)`. This drives the production parent projection,
   `SemioMembers::open`, owner check, map/graph preparation, and atomic commit
   ([`open_child:19984-20032`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19984)).
4. Assert `child_store("content", id)` is `Some(SemioMembers::Flow(_))`, its
   `artifact_ref` equals the parent-declared complete reference, and its persisted `document_id`
   equals the content-addressed id. Drop the borrowed store, then close the target through the
   same bounded production close funnel. This is a real normal-reload proof, not
   `register_child`/test-struct substitution.

Add three hostile cases to that same law: static target id, four-part kind, and wrong slot must
return before `M::open` and leave no child member. The generic neutral four-row composition law
already proves no-publication behavior; Flow's law supplies the first first-party content/member
consumer. It must not claim the later whole-set `LoadChildren` batch, restart recovery, or a
concurrent parent mutation—the current `&mut self` open path intentionally has no safe
interleaving source.

### Registered evidence boundary

After the source patch, run the existing independent oracle exactly as
`bun nx run @semio-tech/flow-plugin:test-source --skip-nx-cache`. The current project maps that
target only to `bun ./📜️script.ts test-source`
([`Flow project:7-15`](../../../../../../✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📋️project.json:7),
[`Flow script:22-26`](../../../../../../✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:22)); it is an AJV/Node
source/corpus gate, not runtime evidence. Register the new exact public Rust FQN in that same
Flow package's normal `test` target and execute it with the repository's exact-one selector
before claiming the restore path. This audit ran neither command and makes no runtime-pass claim.

### Flow coordinate correction supersession — source closed; VCS close remains RED

The preceding Flow identity RED records pre-correction bytes and is superseded on the current
tree. `flow_content_child_from_digest` mints one `child_id` and clones it into
`ArtifactRef.artifact_id` with `s.stdio.semio` / `v1` / `flow`
([`flow artifact:186-190`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:186)). The retained writer
pages that same full spelling into fields 3 and 4 before constructing the target
([`preparation:137-168`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:137)). Both
derived parent declarations now name `s.stdio.semio`
([`snapshot schema:23-30`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:23),
[`aggregate schema:77-92`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:77)); Flow editor and viewer
both expose their derived projection ([`editor:1586-1599`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1586),
[`viewer:49-55`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:49)).

I independently scanned the exact 20 persisted snapshots below. In every current file,
`content.childId == content.target.artifactId` and target dialect is
`s.stdio.semio` / `v1` / `flow`; each brace expansion is two literal files
`{⬅️before,➡️after}/🔣️.json` beneath
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any`:

1. `🧬️schema/🧬️mutations/✂️disconnect-widgets/🧪️tests/🚫️rejects-disconnecting-a-missing-synapse/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
2. `🧬️schema/🧬️mutations/➕️create-widget/🧪️tests/🚫️rejects-a-duplicate-widget-id/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
3. `🧬️schema/🧬️mutations/👯️duplicate-widget/🧪️tests/🚫️rejects-duplicating-onto-a-taken-id/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
4. `🧬️schema/🧬️mutations/📍️move-widgets/🧪️tests/🤖️re-applies-the-current-layout-to-both-widgets/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
5. `🧬️schema/🧬️mutations/🔀️reorder-synapses/🧪️tests/🚪️keeps-the-leading-synapse-at-index-zero/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
6. `🧬️schema/🧬️mutations/🔁️replace-widget/🧪️tests/🟦️replaces-a-note-with-an-identical-note/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
7. `🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/🧪️tests/🌲️re-declares-the-same-endpoints/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
8. `🧬️schema/🧬️mutations/🔌️connect-widgets/🧪️tests/🟫️refuses-a-parallel-synapse-as-a-no-op/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
9. `🧬️schema/🧬️mutations/🔢️reorder-widgets/🧪️tests/🚪️clamps-an-out-of-range-index-onto-the-last-slot/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
10. `🧬️schema/🧬️mutations/🗑️delete-widget/🧪️tests/🚫️rejects-deleting-a-missing-widget/📸️snapshot/{⬅️before,➡️after}/🔣️.json`

The demo's two fields are both `flow-content-877ad0c8ad49fb9b`
([`demo:9-18`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio:9)); the ABA owner fixture uses
`flow-content-reused` for both as well ([`flow test fixture:256-260`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:256)). The coordinator reports
the source-only session 27908 green. I did not run it; this review is current-byte/source-only
and makes no native restore claim.

The material remaining RED is Flow app-owned retirement. `FlowPlayApp` supplies none of the six
document/config/draft owner/disposer methods in its `ArtifactEditor` implementation
([`editor:1572-1611`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1572)). VCS startup therefore installs no
`MemberStoreOwners` for those lanes ([`framework app:19159-19170`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19159)), but close unconditionally drives
the three explicit disposers ([`framework close:23683-23696`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23683)). Missing one returns
`interactive-job.close-owned-disposer-missing` ([`framework helper:13647-13659`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13647)); it is not an
empty-store shortcut.

Do not use generic `bounded_document_store_owners` for Flow document/config: it retires an
entire value in one page ([`framework bounded owner:13398-13457`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13398)), whereas Flow has its own
incremental/terminal-asserting retirement graph ([`Flow retirement:1-57`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:1)).
The smallest safe packet is two editor-visible constructors: (1) `retained::artifact` returns
the exact currently-used tuple `SnapshotRetirementFactory`, `SnapshotRetirementFactory`,
`MutationRetirementFactory`, and `ArtifactStoreCursorDisposer<FlowSnapshot, FlowMutation>`
([`preparation:268-272`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:268)); (2)
`retained::config` returns its same `RetirementFactory` triplet plus
`ArtifactStoreCursorDisposer<FlowConfig, FlowConfigMutation>`
([`config:381-386`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎚️config/🦀️.rs:381)).

`NoDraft` is the genuine empty `NoConfig` type with uninhabited mutation
([`framework:9575-9655`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9575)); it may reuse the existing production
`bounded_document_store_owners::<NoDraft, NoDraftMutation>()` and paired document disposer
pattern ([`sourcing editor:800-815`](../../../../../../✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:800)). The three
disposer methods should be Flow document `ArtifactDocumentStoreDisposer`, Flow config
`bounded_config_store_disposer`, and that paired NoDraft document disposer. The document
adapter delegates to the installed exact owner bundle one bounded item at a time
([`framework disposer:13499-13515`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13499)); it does not erase Flow's
custom retirement.

Until this packet lands, run only native content-identity plus a narrow adapter-projection law
on real default Flow snapshots; that proves editor/viewer forwarding without constructing an app
that cannot close. Afterwards, register a public real-`SemioMembers::Flow` source→envelope
bytes→target `open_child`→bounded `PluginApp::close_step` law. It must assert normal restoration
and terminal empty state; no `forget`, raw drop, or test-only disposer is valid.

### Flow ephemeral close and viewer member authority — current source RED

The pending six editor durable-lane methods are necessary but not sufficient. Once document,
config, and draft close complete, the production close funnel deterministically advances to
presence (stage 3) and transient (stage 4); it does not infer an empty owner from a default value
([`Vcs close funnel:23662-23709`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23662)). Each
stage calls `drive_artifact_owned_disposer`, which rejects a missing disposer with
`interactive-job.close-owned-disposer-missing` before the final interaction lane
([`disposer guard:13647-13660`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13647)).

`FlowPlayApp` currently has a nontrivial `FlowPresence` but supplies no local or peer retirement
factory and no presence or transient disposer ([`Flow editor types and hooks:1572-1611`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1572)). This is a material
ownership boundary: presence contains `Vec<String> preview_off_node_ids` plus camera data
([`Flow presence:17-27`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:17)). It cannot use the Sourcing
no-drop factory: that implementation only safely releases an `Arc` if its payload has no drop
work ([`Sourcing retirement:161-193`](../../../../../../✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:161)).

The smallest correct editor packet is:

1. Add a Flow-owned `FlowPresenceRetirementFactory`, modeled on the real variable-string CAD
   factory rather than a whole-value drop ([`CAD factory:9-76`](../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs:9)). It must first release only an
   externally shared `Arc`; after `Arc::into_inner`, detach the vector, then release one
   `String::into_bytes()` page under the received byte grant. It must retain the root/vector/byte
   cursor until its explicit empty witness; a locally held reader is not permission to move its
   strings.
2. Install that exact factory as **both** local and peer factory. `PresenceStore::begin_retirement`
   requires local authority always and peer authority whenever the detached roster is nonempty
   ([`store admission:196-218`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:196)). Its retained store machinery already drains
   returned local reads, a local root, peer roster entries, and factories one bounded step at a
   time ([`store retirement:79-176`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:79)); no new close loop is warranted.
3. Introduce one first-party `PresenceStoreOwnedDisposer<P, M>` in the framework plugin API,
   parameterized by a domain-provided empty terminal `Arc<P>` and `fn(&P) -> bool`. It should own
   `terminal: Option<Arc<P>>` and `active: Option<PresenceStoreRetirement<P>>`, delegate exactly
   one store-retirement step, preserve the terminal on admission error, and report terminal only
   while the active retirement, store `retirement_started`, empty local terminal, and empty peer
   root all agree. This is the common, non-default portion currently duplicated verbatim by CAD
   and Sourcing ([`CAD disposer:82-135`](../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs:82),
   [`Sourcing disposer:196-237`](../../../../../../✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:196)). It must not supply a generic
   `Default`-means-empty rule or create root retirement factories; those remain domain authority.
   Flow passes `Arc::new(FlowPresence::default())` and a Flow-specific predicate that requires
   empty preview ids and the canonical default camera.
4. Add a framework-owned `NoTransientStoreDisposer` specifically for
   `TransientStore<NoTransient, NoTransientMutation>`. It must reject a zero item grant with the
   standard pending-zero result, assert the state is zero-sized, then complete with the existing
   owner shell. CAD and Sourcing currently duplicate this exact restricted implementation
   ([`CAD:1645-1664`](../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1645),
   [`Sourcing:758-775`](../../../../../../✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:758)). Do **not** generalize it to arbitrary
   transient state: a nonempty transient needs a typed root-retirement and terminal-identity
   contract, not a no-op disposer.

The neutral Flow presence corpus needs empty, one UTF-8 id, an over-page id/list, a local reader,
and a nonempty peer roster. Its independent Bun/AJV oracle should assert exact expected detached
bytes and the `0`-grant / shared-root / terminal sequence; the native Flow law must populate the
actual `PresenceStore`, install both factories, make a peer publication, close through the real
`FlowPlayApp` disposer, and verify no `released_items > 1` or `released_bytes > grant`. A second
native case drives `NoTransientStoreDisposer` with a zero grant then positive grant. Neither law
proves websocket presence delivery.

#### Viewer has two independent production gaps

`FlowApps::FlowViewer` currently erases child resolution to the default
`VcsArtifactApp<ViewerApp<FlowViewer>>`, hence `NoMembers` ([`Flow fleet:10-14`](../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️.rs:10)). The Flow viewer accurately exposes its parent projection, but that
projection cannot open a child through `NoMembers::open`, which deliberately rejects every
dialect ([`NoMembers factory:18115-18122`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18115)).
`PluginBuilder::viewer` is itself hard-coded to that `NoMembers` default
([`builder:387-410`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:387)); unlike its editor sibling, no
`viewer_with_members` exists.

Add `PluginBuilder::viewer_with_members<V, M>` as the direct read-only twin of
`editor_with_members` ([`editor pattern:484-520`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:484)). Its only factory distinction is
`VcsArtifactApp::<ViewerApp<V>, M>::with_registry(...)`; it must retain the existing read-only
capability (`Document/Read` only), stamp the same `V::DOCUMENT_SCHEMA`, and never expose a
mutation builder. Update the Flow viewer enum arm and `.viewer` call to exact
`SemioMembers`. This does not require making `ViewerApp` generic: `M` already belongs to
`VcsArtifactApp`.

There is a broader close correctness gap as well. `ArtifactViewer` offers only preparation and
local-root hooks ([`viewer trait:26379-26470`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26379)), and `ViewerApp` forwards only those
([`viewer adapter:26902-26916`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26902)). It omits document/config owner and disposer hooks,
peer-presence factory, and presence/transient disposer hooks that the same VCS close funnel
requires. Therefore a real viewer VCS instance cannot be certified by the current
`flow_viewer_never_mutates` dispatch-only test ([`Flow test:41-55`](../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️.rs:41)). It must be repaired before a production viewer
open/close claim:

- extend `ArtifactViewer` and `ViewerApp` with the equivalent durable owner/disposer,
  peer-presence, and ephemeral disposer forwarding hooks; keep its draft type framework-owned
  `NoDraft` and give the adapter the existing bounded ZST owner/disposer itself rather than
  asking a read-only author to invent a draft;
- place Flow snapshot-owned retirement constructors in a neutral Flow artifact/retained module
  readable by both editor and viewer, rather than importing the mutation-capable editor from the
  viewer (the viewer explicitly forbids that import);
- use framework no-state presence/transient lifecycle implementations only for the viewer's
  actual `NoPresence`/`NoTransient`, with a local **and peer** no-state factory plus bounded
  disposer. The present code has no production reusable no-state factory/disposer; the similarly
  named variants under `mutation-fixtures` are test-only and cannot be promoted by import.

The acceptance sequence is deliberately split. First exact-one native builder/factory law proves
`viewer_with_members` constructs `VcsArtifactApp<ViewerApp<FlowViewer>, SemioMembers>` and still
rejects a typed mutating verb. Then a public Flow viewer source→child envelope→`open_child`
normal case proves its projection and member factory without write authority. Finally, after the
viewer forwarding/owners packet, a bounded close law proves all six VCS stages terminal-empty.
The existing Flow source oracle and source-only session 27908 do not cover any of these runtime
claims; this audit ran no native gate.

### Flow ownership and content-target reread — editor source closed; viewer remains RED

This section supersedes the immediately preceding editor-presence analysis where it describes
missing Flow durable or presence hooks. It records the current tree only. I performed source
review only; the provider-owned cold target was not used and no native result is claimed.

#### Full child target and digest framing: source PASS

`flow_content_child_handle_bounded` feeds the NUL-terminated domain
`semio.flow.scene.sha256.v1\0` to SHA-256 **before** it serializes the ordered object fields
`widgets`, `synapses`, then `layout` ([`flow artifact:169-181`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:169)).
`flow_content_child_from_digest` then uses one derived id in both `ArtifactChild.child_id` and
`ArtifactRef.artifact_id`, with the complete child coordinate `s.stdio.semio` / `v1` / `flow`
([`flow artifact:183-190`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:183)).

The retained writer follows the identical order: `SceneHash` emits that domain before its
canonical `FlowWorkingScene` reader ([`retained artifact:216-254`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🦀️.rs:216)).
The five neutral Node-crypto rows pin the bytes and hashes, while the native law checks all
three byte grants and rejects substitutions of id, kind, standard, and subset in both editor and
viewer projections ([`identity fixture`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🪪️content-identity/🔣️.json),
[`native identity law:117-151`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📸️snapshot/🦀️.rs:117)).
That is a good source-level full-coordinate/digest proof. It remains unexecuted native evidence.

#### Editor durable and presence closures: source PASS, with one test completion defect

`FlowPlayApp` now supplies the six durable hooks—three exact owner bundles and their three
disposers—for document, config, and draft ([`editor:1593-1619`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1593)). The document/config
paths preserve Flow's domain-specific retirement; only genuine zero-sized `NoDraft` uses the
framework bounded pair. The new three-lane native fixture creates actual stores through
`EditorApp<FlowPlayApp>`, installs the returned owner bundles, and checks its document nested
scene, owned config strings, and empty draft at 1/64/4096-byte grants
([`fixture`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🧹️store-owners/🔣️.json),
[`native law:67-114`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📸️snapshot/🦀️.rs:67)).

The first landing omitted an explicit completion witness: its bounded loop broke on
`PluginCloseStep::Complete` but did not assert that it observed that result. A current reread
supersedes that finding: both the durable and presence loops now set `completed = true` only in
the `Complete` arm and require it before their terminal/released-byte assertions
([`durable law:82-99`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📸️snapshot/🦀️.rs:82),
[`presence law:80-99`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs:80)). This closes the law-soundness issue;
both paths remain source-only until the registered native gate executes.

The shared `PresenceStoreOwnedDisposer<P>` is now exported at
[`plugin API:228`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:228). Its current implementation correctly retains the exact supplied terminal
`Arc`, records the generation after `begin_retirement`, requires terminal pointer identity,
the same generation, a started retirement, the domain empty predicate, and an empty peer root
before it reports terminal ([`generic disposer:8-58`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs:8)). Its single-item delegation respects the caller's byte
limit and preserves the terminal `Arc` on admission failure.

Flow is correctly wired to that generic disposer with separate local/peer
`FlowPresenceRetirementFactory` instances ([`editor:1621-1630`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1621)). The factory takes an exclusive
`Arc` before moving only `preview_off_node_ids` into `FlowRetirement`; the static assertion
proves `CameraJson` needs no drop work ([`Flow factory:8-42`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs:8)). Although the predicate tests only
the vector, the pointer check means the terminal local root is the exact default `Arc`, whose
camera is necessarily canonical; it is not a camera-state bypass. The Flow law uses real local
and peer factories, a captured roster reader, UTF-8 grants, and a terminal test
([`presence law:55-103`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🦀️.rs:55)).

Correction: the generic presence disposer does now exist in the current tree; a claim that it
was absent is historical. The initial claim that there was no production generic
`NoTransientStoreDisposer` is likewise now superseded: it is exported from the plugin API
([`plugin API:230-232`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:230)) and
`FlowPlayApp` installs it ([`editor:1633-1635`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1633)). It is correctly
closed over the exact `TransientStore<NoTransient, NoTransientMutation>` alias, requires both
types to be zero-sized/no-drop, replaces only after a positive grant, and rejects a later
terminal pointer or generation mismatch ([`adapter:7-40`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement/🦀️.rs:7)).

The Flow four-step fixture now proves zero item, zero byte, positive retirement, sticky
completion, a foreign-owner rejection, and—after `reset(NoTransient::default()).await`—a
same-owner generation rejection before a fresh disposer can close the reset store
([`native law:7-43`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧪️tests/🦀️.rs:7)). This closes the
generation-law gap. The cross-platform assertion now compares `Store` with the corresponding
tuple, avoiding the previous field-sum padding failure. It is still not a Rust `repr` guarantee,
so safety must rest on the exact typed alias and ZST/no-drop assertions rather than an assumed
layout. No native execution is claimed.

#### Viewer document retirement: current API RED

`FlowViewer` has the real `FlowSnapshot` and projects its `content` child correctly, but has no
way to supply document owners or a document disposer ([`viewer:39-58`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:39)). This is not merely a missing
Flow implementation: `ArtifactViewer` declares preparation and local ephemeral factories but
no `build_document_store_owners` or `build_document_store_disposer`
([`viewer trait:26424-26447`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26424)), and `ViewerApp` consequently forwards none
([`viewer adapter:26906-26920`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26906)). It inherits `ArtifactApp`'s `None` defaults.

The minimal clean packet is a document-only pair of default-`None` hooks on `ArtifactViewer`
and direct forwarding in `ViewerApp`: `build_document_store_owners` and
`build_document_store_disposer`. Move the current Flow snapshot retirement/owner factory out of
the mutation-capable editor subtree into a neutral Flow artifact-retained module, then let both
editor and viewer call it. Do not import `editor::flow` from the viewer: that violates its
explicit read-only dependency boundary. A native viewer-specific law must use
`ViewerApp<FlowViewer>`, construct a snapshot from the same content-identity corpus, install
that document owner bundle, run the exact viewer disposer to an explicit `Complete`, and then
assert terminal empty. It should not use an editor adapter or test-only closer.

The source gate now registers five current native laws under
`child-identity-check` ([`Flow script:22-44`](../../../../../../✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:22)). Add the viewer close FQN only after the neutral extraction lands; this audit
did not run the registered command.

### Exact neutral Flow document-retirement extraction inventory

The viewer must neither import `editor::flow` nor duplicate the editor's close machine. The
smallest reusable boundary is a **document-only** common module declared by the Flow artifact
root, for example
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs` exposed as
`crate::artifacts::flow::retained`. It has no editor/viewer/UI dependency and imports only
`FlowSnapshot`, `FlowMutation`, and `FlowWorkingScene` from `crate::artifacts::flow`,
`flow::retained::{FlowOwner, FlowRetirement}`, `flow::FlowLayoutEntry`, `store`, and `std`.
The physical name may follow an equivalent existing common artifact taxonomy, but it must be
declared from the artifact root—not re-exported from the editor module.

Move, rather than copy, these current document lifecycle surfaces:

1. `SnapshotRetirementFactory` and its `SnapshotRetirement` state machine from
   [`editor snapshot retirement:8-65`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/📸️snapshot/🦀️.rs:8).
   It must retain the exact root/owned snapshot, consume the local `FlowWorkingScene` once,
   and page schema, child id, target id, kind, standard, and subset to an explicit empty
   witness.
2. The document subset of `Owner`/`Retirement` from
   [`editor retained:17-263`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:17): only byte owners, a
   `FlowRetirement` owner, a `FlowMutation` owner, and `Vec<FlowLayoutEntry>` are needed.
   On snapshot scene extraction, push framework `FlowOwner::Widgets`, `Specs`, and `Layouts`.
   On mutation extraction, retain the exact ten `FlowMutation` variants using the current
   `mutation` mapping ([`editor retained:244-262`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:244)); do not pull
   Flow config owners, app command preparation, `SceneCopy`, or mutation recipe execution into
   this common module.
3. `MutationRetirementFactory` from
   [`recipe:15-45`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs:15), rebuilt over that common
   document retirement. A viewer must decode and later retire historical `FlowMutation` values;
   that does not grant it mutation emission.
4. One common `document_store_owners()` assembling the two snapshot factories, the mutation
   factory, and `ArtifactStoreCursorDisposer<FlowSnapshot, FlowMutation>`, replacing the
   editor-private [`store_owners:16-27`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🦀️.rs:16).

Then add precisely two default-`None` hooks to `ArtifactViewer` and their direct `ViewerApp`
forwards: `build_document_store_owners()` and `build_document_store_disposer()`. `FlowViewer`
uses the shared `document_store_owners()` and a normal
`ArtifactDocumentStoreDisposer<FlowSnapshot, FlowMutation>`. Do not move editor-only config,
draft, presence, transient, tool, or preparation hooks in this packet. The viewer's types are
currently `NoConfig`, `NoPresence`, and `NoTransient`, while its only nontrivial closing owner
is the document store ([`viewer:39-51`](../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:39)).

Acceptance must add a public native viewer law—not an editor fixture—that constructs a
`ViewerApp<FlowViewer>` document store from one of the existing five neutral content identity
rows, installs the viewer-returned common owner bundle, drives the viewer document disposer
with 1/64/4096-byte grants, asserts an explicit `Complete`, and confirms terminal-empty. The
same fixture must include a local content scene and an encoded historical mutation so both
common factories are used. Its sibling negative substitutes one target coordinate and requires
the existing projection admission to fail before the store factory opens a child. Extend only
the Flow `child-identity-check` exact FQN list after this source first proves RED, then turns
green. This remains a blueprint; no implementation or native run occurred in this audit.
