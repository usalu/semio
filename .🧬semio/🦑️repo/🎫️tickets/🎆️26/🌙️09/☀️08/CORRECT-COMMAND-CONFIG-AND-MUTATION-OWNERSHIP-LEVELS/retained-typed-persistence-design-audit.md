# Retained Typed Persistence Slice A Design Audit

**Audit date:** 2026-09-13  
**Scope:** Read-only design audit of the proposed typed Generation2d/Generation3d snapshot and mutation persistence slice. This audit did not run Cargo or any tests, and made no production changes.

## Verdict

**Do not authorize a typed Slice A start yet.** The schema defines the right boundary, and the accepted retained inflater integration supplies the lower mounted-session protocol that Slice A must extend. Sol can resume Pack work, but Slice A first requires a shared Flow collection/string retirement correction, followed by its consumer validation; then it can implement the owner-and-ledger contract below. A Generation3d-only compensation, fixed byte-count adapter, or local `4096` replacement is not acceptable.

The resulting Slice A ledger is an exact-capacity **local owner ledger**. It is not, and must never be described as, a process allocation credit. A general process working-allocation authority remains a separate, unimplemented prerequisite for any process-wide admission claim.

## Decision Record

| Area | Verdict | Evidence |
| --- | --- | --- |
| Typed snapshots and mutations | Current code is not a retained physical owner. | Both generations eagerly reserve parser stacks, construct ordinary `String`/`Vec` candidates, return naked typed values through `take`, and report only lower Pack-body allocation bytes. |
| Typed handoff | Fails Slice A. | The mounted session returns `Generation*dSnapshot` / `Generation*dMutation` without the capacity ledger; field authorities then publish a plain value. The receiving retirement owner cannot retire the physical backing it was never given. |
| Generation2d retirement | Fails Slice A. | Typed snapshot/mutation close paths clear and drop ordinary backing; their mounted close results report zero released bytes. |
| Generation3d retirement | Fails Slice A and has a grant bypass. | Its snapshot owner calls `retirement.close_step(1, 4096)` internally. `4096` is not a named page limit or a caller grant. |
| Larger G3 actual allocations | Cannot receive an exact physical release in the current path. | `FlowRetirement::close_step` uses the supplied byte argument to truncate `Vec<u8>` by logical length. It can make repeated 4096-byte logical progress, then drops the vector/string capacity unreported. |
| Accepted inflater integration | Compatible dependency; no re-test needed for this audit. | The current working-tree diff in both snapshot sources is limited to retained segment cursor construction, demand/reserve ordering, allocated-byte aggregation, and close/release forwarding. It contains no typed-owner work. |
| SPR/history (Slice B) | Remains explicit and deferred. | The schema requires record-atomic retained history ownership. Current `RetainedHistoryDecode` and member-open still use ordinary history/dictionary/edit-id/vector backing and a full `Vec::with_capacity(total)` history copy. |
| Generic `WindowConfig` loading | No Slice A change. | Its owner trait and registry load a local window configuration by registered kind and `parse_document_pack`; they do not own the member/archive retained operation. Artifact/history requirements must not be added to that generic API. |

## Required Prerequisite: Shared Flow Collection and String Retirement

This correction belongs **before** typed Slice A because the faulty retirement primitive is shared, and the Generation3d path merely exposes it. The implementation owner is the framework Flow retained module:

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`, specifically `FlowOwner` at lines 17-40 and `impl ErasedSnapshotRetirement for FlowRetirement` at lines 100-208.

`FlowRetirement` currently owns direct `Vec`/`String` backing through `Bytes(Vec<u8>)`, `Strings(Vec<String>)`, `Widgets(Vec<Widget>)`, `Specs(Vec<SynapseSpec>)`, `Neurons(Vec<Neuron>)`, `Synapses(Vec<Synapse>)`, `Previews(Vec<FlowPreviewGui>)`, and `Layout(Vec<FlowLayoutEntry>)`. The direct vector arms pop logical elements and requeue the residual `Vec`, but never release the vector allocation capacity. `Bytes` additionally truncates logical length according to the supplied grant. Converting a `String` to `Vec<u8>` does not repair this: the backing capacity still persists until a later unreported drop.

The shared implementation must introduce a physical collection/string retirement primitive that retains each backing's actual capacity and its child retirement frontier. It must retire children first where necessary, then release the collection or string backing atomically only when the caller's byte grant covers that backing's actual capacity; it reports that exact capacity once and drops the backing in that step. A smaller caller grant returns `Blocked` or an unambiguous no-progress result without truncation, reallocation, hidden drop, or fabricated release. It must use checked `capacity × size_of::<T>()` accounting for typed collections and `String::capacity()` for UTF-8 backing. Ordered containers may continue through their own physical cursors only if their release semantics meet the same rule.

This is deliberately a shared primitive correction, not a Generation3d wrapper and not a ledger adapter that guesses a byte total around `FlowRetirement`. The Generation3d snapshot retirement at `.../generation3d/.../mutations/💾️binary/🦀️.rs:673-705` already delegates its fixture to that shared primitive. After the correction, its mounted close path must propagate the actual caller `maximum_items` and `maximum_bytes` through the Generation owner and Flow retirement unchanged. No layer may manufacture `4096`, split one physical allocation into logical-length pages, or claim the shared backing was released when it remains allocated.

The current `FlowRetirement::retire_cold` loop at lines 53-56 also cannot remain a fixed-4096 close caller if the corrected primitive requires an actual-capacity grant. It needs a cold-only unlimited/next-required grant strategy that still uses the same physical ownership frontier; retained callers must never use that escape hatch.

### Shared-change validation surface

The correction is shared by more than Generation3d. Validate these consumers after it lands, keeping the work separate from typed Slice A implementation:

- Generation2d and Generation3d replay and snapshot retirement factories, which construct `FlowRetirement` for fixture disposal.
- Flow plugin scene, mutation, snapshot, presence, canonical retained editor, recipe, and preparation retirement paths. Some existing paths treat `Blocked` under a positive grant as unreachable; they must either propagate `Blocked` or request the true next physical release capacity.
- Framework Flow VCS schema retirement/diff/operation paths and Flow host retirement paths, which retain Flow values through the same frontier.
- Store hydration, publication preparation, and canonical-edit callers that enforce `released_bytes <= maximum_bytes`; they must receive the unchanged caller grant and preserve a blocked nested owner without dropping it.
- The framework Flow retained unit suite plus each affected consumer's existing retirement tests, including a string and each direct vector variant with actual capacity above 4096, an undersized grant, an exact grant, and terminal emptiness.

## Contract Sol Must Implement for Slice A

### 1. One physical owner carries a typed value and its exact local ledger

Each Generation2d/Generation3d snapshot candidate and mutation candidate needs one non-cloneable internal owner package. It contains:

- the typed value while it is being built or after it is ready;
- every typed physical backing owner and an aggregate actual-capacity ledger;
- a state frontier such as `Building`, `Ready`, `Transferred`, or `Retiring`;
- retained close state capable of returning `Blocked`, `Pending { released_bytes }`, and `Complete`.

The package may expose a typed value only by an atomic ownership transfer. It must not offer the present `take() -> Snapshot` / `take() -> Mutation` escape hatch. A receiver must accept the value **and the same ledger in one operation**, prove adoption, and only then may the source mark itself transferred. If publish/admission refuses or is stale, the source retains the unchanged package and closes it itself; no naked value, duplicated ledger, or unaccounted drop is permitted.

The receiving snapshot/mutation retirement authority must therefore accept an owner-bearing typed package, or an equivalent indivisible private package under the exact Generation authority. The existing plain `publish_snapshot_reserved` route and the plain artifact-envelope retirement inputs do not meet this contract. This is a targeted capability addition to the Generation snapshot/mutation authority, not a general store, document, or WindowConfig capability.

### 2. Every nested `String`, `Vec`, and list-like backing becomes an explicit physical owner

The builder constructors must allocate nothing. A retained value cannot consume an ingress token until it has requested, received, and checked the one exact reserve that creates its next physical backing. Requested bytes are a logical request; admitted actual capacity is checked against the local actual-capacity limit using checked byte arithmetic and remains recorded even if it exceeds the request. Zero or subexact grants leave all candidate state, pointers, lengths, capacities, and token ownership unchanged and surface the first sticky fault.

This applies to every live backing, including:

- typed container, JSON, and DSL stacks, their frame payloads, and active scalar string;
- snapshot fixture schema, widgets, synapses, layout/ordered-map backing, camera and all strings nested below them;
- Generation root/map/list backing, neural dictionary entries, sequences, tables, table rows, and presence vectors;
- JSON/DSL arrays, objects, rows, map values, and their strings;
- mutation stack, JSON/DSL stack, the three retained mutation strings, and all optional widget, synapse, layout, camera, generation, and dynamic value payloads;
- the final snapshot/mutation values after completion, including list-like backing that survives parser-frame removal.

An owner may use a common retained contiguous/string/list primitive, but each allocation must remain attributable to the particular candidate package. It may not hide behind a parent `Vec`, a later conversion to an untracked `Vec`, or a fixed static byte charge. The aggregate includes all simultaneous typed backing. At the mounted-session level it is added to the existing source, segment, Pack-value, and catalog aggregate exactly once.

Fresh initialization, copy/resume, and replacement paths are part of the same chain: a copied string/vector/list is admitted before allocation and registered in its destination owner; a displaced live value moves into retirement with its own ledger before replacement becomes visible. They may not use direct `try_reserve_exact` as an unledgered side path.

### 3. Preserve a pending typed token and extend the accepted mounted-session protocol

Today a Pack value grant is immediately fed to the ordinary typed owner. Once typed construction can block on physical allocation, the mounted session needs a retained `pending_typed_token` (or equivalent) which owns the yielded typed token until the typed owner has reserve authority. It must:

1. expose the typed owner's next exact allocation request while that token is pending;
2. reserve and register the actual capacity before calling typed acceptance;
3. consume/release the token only when typed acceptance succeeds;
4. keep the token and candidate unchanged on zero/subexact/rejected reserve, cancel, or stale admission.

Use the accepted lower-session ordering: a segment allocation that blocks compressed ingress remains first; after a typed token has been produced, the pending typed allocation precedes additional Pack-value/catalog progress. This prevents consuming or replaying a token while its recipient cannot own the next backing. The session's demand, reserve, allocated-byte total, close result, and next-release query must include the typed package without removing or double-counting the accepted segment owner.

### 4. Retirement is grant-driven, exact-capacity accounting

Replace the current `close_step() -> bool` typed close with a close frontier taking the caller's item and byte grants. It reports actual released backing capacity, leaves deferred work pending when the grant cannot cover the next release, and is terminal only when no candidate, parser frame, string, list/vector/map backing, token, or ledger entry remains.

The required close order is:

1. pending typed/Pack input token;
2. partial scalar and parser-frame logical contents;
3. typed child logical owners;
4. typed physical backing with actual released capacities;
5. existing history/Pack physical owners in their established session close order.

After a successful transfer, the mounted session retires only its residual Pack/frame owners. The recipient's retirement owner retires the transferred typed package. Each backing therefore has one retirement authority and one release event.

Generation3d's hard-coded `4096` must be removed from this retained path only **after the shared Flow primitive above provides correct physical release semantics**. In `generation3d/.../snapshot/binary/🦀️.rs:1011`, `retirement.close_step(1, 4096)` manufactures a byte grant in a caller-owned close operation. `FlowRetirement` labels its own `retire_cold` loop as cold-only, but accepts the same argument as `maximum_bytes` in retained `close_step`. Its `Bytes(Vec<u8>)` arm reports `min(maximum_bytes, bytes.len())` and truncates logical length; when the resulting vector is eventually dropped, its allocation capacity is not reported. A value with a capacity larger than 4096 can therefore appear to progress through logical chunks yet never release its actual allocation in the ledger. This is a caller-grant bypass and an accounting loss, not a local page ceiling. Do not compensate in Generation3d: propagate the caller grant unchanged to the corrected shared owner.

The Generation2d direct drop and the Generation3d flawed retirement are baseline asymmetry and current typed-owner debt. Slice A removes both rather than adopting either behavior.

### 5. Rejection, cancellation, stale cleanup, and authority boundaries

The existing field authorities already check cancellation/staleness before driving sessions. Slice A must retain that diagnostic behavior while making all terminal paths physical-owner safe:

- rejection before reserve: no typed allocation and no token consumption;
- subexact/zero grant: sticky first fault, no mutation of capacity, pointer, length, ledger, or token;
- cancellation/stale after candidate creation: transition the package to the above close frontier and emit all actual releases under caller grants;
- publish refusal: retain the package and its ledger together for retry or close;
- success: transfer the package exactly once; source and receiver terminal predicates make duplicate retirement impossible;
- stale/cancel races: no fresh initialization/copy allocation may occur after the authority denies admission.

The capability boundary is narrow. Parser/mounted-session code may build and close its package; only the Generation snapshot/mutation field authority may atomically publish it; only the exact recipient/retirement authority may own it after publication. A generic `WindowConfigOwner`, registry, generic document load, or generic artifact config factory must not gain a typed-builder, member-history, archive, or process-credit capability for this work.

### 6. Do not turn local allocation accounting into process credit

The local typed ledger establishes that one mounted candidate owns specific collection capacities, and that those capacities are released by its retirement frontier. It can bound the session's admitted typed backing and make handoff auditable. It cannot reserve, spend, or return process-wide working-memory credit.

The current process design has no general shared working-allocation authority: `Fem3dBackingCredit` is per-instance and `JobPayloadOperationLedger` covers fixed payload streaming rather than arbitrary typed working allocations. A later process authority must issue generation-keyed requested/reserved/actual leases that follow owners across handoff, cancellation, and retirement. Slice A must name its counters `local`/`owner` allocation bytes and must not report them as process budget recovery.

## Slice B Obligations Preserved by Slice A

Slice A neither implements nor weakens the retained SPR/history contract. The `retainedTypedPersistence` schema requires Slice B to retain a record-atomic stream and explicit owners for history-byte copy, dictionary, edit-id index, records, and payload bytes; its order remains verify, admit, decode, validate, typed replay, then retire. It also requires no batch decode, sticky first-fault behavior, and the documented terminal close order.

Current source shows why it remains a separate slice: `RetainedHistoryDecode` owns an ordinary `HistoryLog`, `DictReader`, and `Vec<String>` edit IDs, and member open allocates `Vec::with_capacity(total)` for the whole history before decoding. Those are not Slice A implementation shortcuts. Slice A must leave them visibly deferred while ensuring its retained mutation package has the exact owner-bearing typed replay boundary that Slice B will later call.

The generic WindowConfig load remains unchanged. Its current registered-kind config parsing is not an artifact/member retained history path. No Slice A type, owner, limit, decode mode, document identity, or retirement capability may leak into `WindowConfigOwner`, `ErasedWindowConfigStoreOwner`, their registry, or generic `parse_document_pack` because of the artifact requirement.

## Bounded Acceptance Criteria Before Calling Slice A Complete

1. The shared Flow collection/string primitive is corrected and its validation surface passes: every direct Flow `String`/`Vec` backing above 4096 blocks below actual capacity, releases exactly once at an adequate caller grant, never truncates logical length as a release surrogate, and leaves no unreported final drop.
2. Native retained tests prove allocation-free candidate construction for both generations and for snapshot and mutation builders.
3. Tests enumerate each listed `String`, `Vec`, map/list, row, presence, JSON, and DSL backing; every creation requests/reserves an exact capacity, records actual capacity, and checks the local aggregate actual limit.
4. Tests prove a pending typed token is retained across zero/subexact allocation, cancellation, stale rejection, and reserve failure, with no ingress replay or state mutation; accepted segment ordering remains intact.
5. Tests prove snapshot and mutation transfer moves value and ledger together. A plain typed value cannot be published, retained, or retired through the new path without its ledger.
6. Tests drive close with multiple caller byte grants, including an actual string/vector allocation above 4096, and observe exact capacity releases with no internally manufactured byte grant and no final unreported drop.
7. Tests cover rejected, cancelled, stale, replacement, copy/resume, and post-transfer close paths, proving one retirement owner and zero residual typed allocation at terminal state.
8. Session aggregation proves typed actual bytes are included once alongside the accepted source/segment/value/catalog owners; no accepted inflater behavior is removed or re-tested as part of this slice.
9. Source-law/schema checks keep `retainedTypedPersistence` as `next-slice-contract`, preserve the Slice A/B order and owner list, and preserve the explicit Slice B deferred obligations.
10. Review confirms no Slice A edit changes generic WindowConfig loading and no Slice A metric claims general process allocation credit.

## Evidence Inspected

- `retained-pack-inflater-physical-implementation.md` — accepted retained segment allocation/close integration and its stated typed/history scope boundary.
- `retained-typed-snapshot-mutation-and-spr-history-design.md` — planned Slice A and B contract.
- `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json:329-380` and matching fixture/source-law test — `retainedTypedPersistence` contract.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/.../snapshot/💾️binary/🦀️.rs:152-183,957-981,1110-1447` — eager typed snapshot backing, naked handoff, absent typed session ledger, direct close/drop.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/.../snapshot/💾️binary/🦀️.rs:171-215,989-1040,1303-1497` — same baseline plus flawed hard-coded-4096 retirement call.
- Generation2d/Generation3d mutation binary owners at `.../mutations/💾️binary/🦀️.rs:920-959,1677-1918` — eager stack backing, naked mutation handoff, lower-body-only accounting, zero-byte typed close.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:17-208` — shared `FlowRetirement` owner variants, cold-only 4096 helper, direct vector/string retirement, and grant semantics.
- Flow retirement consumer search — Generation2d/3d factories, Flow plugin scene/mutation/snapshot/presence/canonical/recipe/preparation paths, framework Flow VCS and host paths, and Store retirement callers all require post-change validation.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:1449-1776` and member-open operation source — current ordinary Slice B history backing and whole-history copy.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:12-23,357-365,465-476,572-574` — local WindowConfig owner/load scope.
- `fem-process-backing-owner-boundary-review.md` and `allocation-budget-semantics.md` — local capacity checks are not a general process allocation authority.

The audit only reviewed the accepted inflater evidence and the current diff needed to establish its integration boundary. It did not repeat its test suite.
