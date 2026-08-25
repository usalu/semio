# P7b Puzzle2d Mounted Retained Fill Repair

Date: 2026-08-25
Executor: Sol High
Verdict: source/static implementation complete; independent audit required; compiler, runtime, native/Wasm, and watchdog execution remain deferred.

## Scope and prerequisites

The implementation follows the P7b mounted retained fill contract and the accepted P2a1/P1 worker/session ownership protocols. It changes only the Puzzle2d board-fill core, its Puzzle2d command/config/schema/UI/fixtures, and this P7 ticket report. It does not change Energy, actor, shard, renderer, P8, or the shared root `📜️script.ts`.

The mounted route is now:

`setFillCount/brushFillSessionBegin → fixed registry reservation → generation-qualified SnapshotRead transfer → MountedWorkerJobSession<ArtifactBoardFillJob> → worker-only fixed-page capture → BoardFillJob → shared InteractiveNative process WorkerPool → typed preview/checkpoint/CommitCandidate → typed paged document apply → incremental terminal-empty close`.

## Changed-file inventory

1. `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs`
   - Replaces the whole fill snapshot/checkpoint/JSON-placement implementation with fixed-page typed owners, a worker-facing fixed ingress, retained per-field capture/source/accept cursors, CommitCandidate result encoding, exact placement handle identities and edge kind, fallible actual-capacity page admission, and exact incremental close.
2. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
   - Mounts SnapshotRead preparation/reconciliation and routes every fill continuation through an early branch before whole ArtifactView clone, BoardHost rebuild, JSON parse, and whole-document diff. The fill branch now constructs only the minimal runtime/effect/typed-mutation/operation authority; it has no `BoardHost`, `RefCell`, or default host backing.
3. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs`
   - Owns the fixed eight-slot generation-qualified session registry, minimal fill action authority, fallible node backing, worker-owned SnapshotRead capture wrapper, retained node/handle/kind/template/rule field and byte subcursors, shared-pool mount, exact CommitCandidate consumption/ACK, checkpoint adoption, per-field typed schema apply with same-turn destination credit, cancellation/retry/discard, and abandonment close.
4. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-begin/🦀️component.rs`
   - Requires exact typed count and seed admission.
5. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-clear/🦀️component.rs`
   - Routes clear through exact session discard.
6. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-step/🦀️component.rs`
   - Decodes only the generation scalar and forwards it through the minimal fill authority.
7. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active-utility/🦀️component.rs`
   - Cancels and drains displaced fill ownership instead of resetting detached scalar state.
8. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
   - Adds the Copy-only fill runtime projection and fixed 64-byte stage/fault identifiers. Fill continuations copy only these admitted scalars, emit a typed `Fill` event, and never clone or ordinarily drop the dynamic config's candidate vector, maps, or strings.
9. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs`
10. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️component.json`
11. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️component.ts`
12. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️component.graphql`
13. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️component.proto`
    - Replace byte checkpoint/JSON preview state with the cross-platform typed lifecycle, operation, generation, base revision, checkpoint sequence, progress, stage, and fault projection. Accepted count is fixed-width `u64` at the schema boundary.
14. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs`
    - Adds accessible localized EN/DE progress, cancel, retry, fault, and result controls/states.
15. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs`
    - Adds the corresponding EN/DE terminology.
16. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🦀️component.rs`
   - Migrates fill fixtures to the mounted worker path, consumes terminal CommitCandidate directly, and witnesses the exact edge-kind scalar plus all other retained acceptance stages.
17. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/📓️sol-p7b-puzzle2d-mounted-retained-fill-repair-2026-08-25.md`
    - Records the exact implementation inventory, ownership laws, hostile fixtures, residual census, static gates, and deferred shared-verifier work.

## Terra RED remediation

| Independent blocker | Production closure | Exact evidence |
| --- | --- | --- |
| Whole ArtifactView clone/parse/rebuild/diff on every continuation | All fill actions return through a dedicated branch before the ordinary `doc.snapshot.0.clone`, `sync_host_fixture_content`, `parse_fixture_v1`, and `puzzle2d_document_delta_operations` route. Prepared placement emits typed mutations directly. | The live branch production slice has zero matches for all four whole-work families; clone/rebuild/diff restoration mutations fail the permanent local predicate. |
| Capture on the action/UI route | `mounted_job_prepare_snapshot_read` and `pending_effects` transfer one exact immutable store lease into `ArtifactBoardFillJob`. Its InteractiveJob implementation advances capture and inner WFC only through `MountedWorkerJobSession::pump_one` on the shared Interactive pool. | The action production slice has zero `BoardHost`, legacy capture owner, or `capture.step` matches and contains the SnapshotRead, canonical freshness, fuel-one, 7 ms, shared-pool, and mounted-wrapper witnesses. |
| Complete candidate discarded in favor of mutable job state | `BoardFillResult` is encoded into the CommitCandidate output. The live consumer decodes only `StepOutcome::Complete(candidate)`, retains that exact outcome, and incrementally ACKs its state/output before terminal adoption/close. | Production `BoardFillJob::take_result` count is zero; candidate match and decoder counts are exactly one each; the mutable-result restoration mutation fails. |
| Eleventh page insertion discarded `Err(candidate)` | Both candidate insertions bind `Err(candidate)` and restore the exact owner into `compatibility_candidate`; the same law covers every ingress/source/virtual/placement page. | Fill-region census is exactly 11 insertions and 11 explicit returned-owner bindings, zero `.is_err()` discard; the last compatibility-site mutation fails. |
| Fill continuations constructed and ordinarily dropped a populated `BoardHost` | `Puzzle2dFillActionCtx` exposes only the retained runtime projection, effects, typed artifact mutations, and exact operation authority. Every fill wrapper and dispatcher uses it directly; `setFillCount` publishes its bounded tool effect without a host. | The live fill branch and complete action production slice each have zero `BoardHost`, `RefCell`, or `BoardHost::default()` matches. Restoring a default host fails the permanent branch predicate. |
| Artifact capture coalesced multiple fields and whole strings per fuel grant | Node, handle, kind, template, and rule capture now retain explicit field enums plus a byte cursor. The ingress exposes separate `begin_*`, one-byte `push_*_byte`, scalar `set_*`, and `publish_*` opportunities. The wrapper invokes `capture_one` once and rechecks cancellation, operation/generation, SnapshotRead authority, and deadline around it. | The capture slice has seven byte-cursor sites, 39 exact ingress opportunities, zero loops/iterators/clones/string construction, and zero coalesced `push_node/push_handle/push_kind/push_rule` calls. Field-coalescing, whole-text, ingress-array, and ingress-whole-text mutations all fail. |
| Every fill continuation cloned and snapshotted the whole dynamic `Puzzle2dConfig` | The mounted branch now copies a `Puzzle2dFillRuntime` containing eleven fixed-width scalars/enums and two fixed 64-byte text slots. `Puzzle2dFillActionCtx` accepts only that projection. Changed state emits `Puzzle2dConfigMutation::Fill`; unrelated candidates, maps, strings, camera, engagement, and utility owners never enter the continuation. | The exact branch census is zero `config.clone()`, `Puzzle2dConfigMutation::Snapshot`, `Vec<Value>`, and `BTreeMap`, with one fixed projection read and one typed fill mutation. The context and projection slices independently reject full-config, dynamic-vector/map/string, whole-snapshot inverse, and injected-clone mutations. |

## Retained work and ownership laws

| Authority | Admission and bounded opportunity | Refusal/stale/cancel/fault ownership | Terminal close |
| --- | --- | --- | --- |
| Registry slot and node backing | One of eight atomic slots is claimed before a fallible exact `FillSessionNode` backing allocation. | Reservation `Drop` clears a pre-publication `LOCKED` slot; checked-out guard `Drop` republishes the exact app/operation/generation owner. | Only a terminal-empty node clears its slot and deallocates its backing. |
| SnapshotRead and capture | The action reserves only the registry authority. `pending_effects` transfers one exact generation-qualified SnapshotRead into `ArtifactBoardFillJob`; its InteractiveJob step admits exactly one source scalar, character, edge item, retained owner, or fixed-page publication per shared-pool worker grant. | Fuel, deadline, cancellation, operation/generation, render generation, and canonical revision are checked around every capture unit. Fault/stale/cancel retains the exact read, ingress, partial text/record, field/byte cursor, and inner job owners. | The inner job, ingress record/page/scalar, SnapshotRead return witness, and fault each close incrementally; Drop requires terminal-empty. |
| Source and candidate construction | Source ID, node kind, handle kind, wire kind, edge kind, slot, weight, and source publication have distinct retained stages. Candidate acceptance separately advances node ID, edge ID, node kind, source handle, target handle, icon, virtual node, source connection, one template, one generated handle ID, one virtual handle, and one handle publication. | Any missing/stale source, candidate, template, or capacity witness leaves every partial typed owner in `BoardFillJobState` for the common close cursor. | Partial source/template/ID/virtual/placement owners are retired one at a time before fixed pages. |
| Fixed pages | Each page fallibly reserves exactly four logical items and records the allocator-reported capacity bytes. Page count is retained, so retirement and terminal checks are O(1), without scanning the fixed page table. | Same-owner `try_push_owned` returns the exact draft/item on MAX+1 or backing-allocation refusal; every node, handle, kind/template, rule, source, candidate, and placement caller restores that owner in its retained cursor. | Close pre-credits the actual retained bytes and releases exactly one empty backing page. |
| Mounted fill session | `MountedWorkerJobSession<ArtifactBoardFillJob>` uses the shared `InteractiveNative` process pool, `fuel_per_step = 1`, and `step_budget_ms = 7`; capture and WFC both remain behind the same worker authority. | Submit refusal remains in the mounted session and retries; cancellation, stale operation/generation, deadline, worker fault, and checked-out fault retain an exact closeable wrapper/session/job owner. | Outcomes are ACKed incrementally; the checked-out wrapper, inner job, ingress, and read lease then close through the accepted session protocol. |
| Preview/checkpoint/terminal | Preview is one replaceable typed scalar. Checkpoint is a single lossless typed state owner and pauses the worker until adoption. `StepOutcome::Complete(CommitCandidate)` is the sole result owner and is decoded while the outcome remains retained. | A rejected checkpoint adoption returns the exact checkpoint and converts it into a detached closing job; no byte serialization, mutable `take_result`, or ordinary Drop is used. | Commit payload/state are ACKed incrementally before the terminal scalar and session are adopted and closed. |
| Placement apply | Each exact handle ID/kind/angle/radius, node field, edge ID/kind/source/target, fixed-handle transfer, and publication is a retained continuation. Handles stage in a fixed 16-slot typed array. The live output vector is credited and allocator-bounded in the same Publish turn before the node or edge moves; both mutations publish atomically. | Allocation/field/schema refusal retains the exact typed placement plus partial handle/node/edge owners. Operation/generation/canonical revision freshness is checked before every continuation. | Close removes one handle field, fixed handle, node field, edge field, or typed placement page per action; no partial JSON owner exists. |
| Replacement and abandonment | New operations have real app/operation/generation/base-revision identity and can occupy a distinct fixed slot while displaced work drains. | Stale/cancel/discard and lost handles enter the same registry close pump; `u64::MAX` does not alias generation zero. | Each abandonment pump closes at most one owner/item/page before republishing or retiring it. |

## Hostile laws and fixtures

The scoped sources now contain laws for:

- deterministic mounted replay and typed checkpoint restore;
- stale typed checkpoint refusal with exact handback;
- cancelled and superseded mounted owner close;
- deadline yield before a semantic unit;
- worker shutdown refusal and an independently saturated 1,024-entry lane returning the exact session for close;
- completed-but-unclaimed terminal close;
- identical results across worker counts 1, 2, 4, and platform default;
- a large fixed-capacity host with substantive cursor progress and a watchdog-overrun oracle;
- capture MAX+1/backing refusal with exact draft handback and incremental page close;
- an exact mounted trace proving every candidate acceptance string/template/virtual/publish stage is a separate fuel-one preview opportunity;
- registry MAX/MAX+1 reservation, pre-publication panic recovery, post-publication lost-guard recovery, stale-generation rejection, and terminal-generation non-aliasing;
- a production-slice law proving the fill branch precedes and excludes whole ArtifactView clone, BoardHost sync/parse, and whole delta derivation;
- a production-slice law proving SnapshotRead capture remains inside `ArtifactBoardFillJob`, with shared-pool mount, fuel one, and canonical freshness;
- exact CommitCandidate-only result consumption, same-turn placement destination credit, and all 11/11 `try_push_owned` returned-owner handbacks;
- twelve faithful hostile mutations that restore the audited whole clone/rebuild/diff/default host, remove worker capture, restore mutable terminal reread, remove same-turn credit, discard the compatibility owner, coalesce two source fields, copy whole text, or restore whole-record ingress APIs, each rejected by its live source predicate;
- five additional faithful mutations that inject a whole config clone, restore whole-config fill snapshot publication, add dynamic `Vec<Value>` authority to the action context, add dynamic vector/string backing to the fill projection, or restore a snapshot-based fill inverse, each rejected by the live predicate;
- fixed fill text MAX/MAX+1 capacity and a source law proving the fill projection contains no `Vec`, `BTreeMap`, or `String` backing;
- localized German progress/cancel/fault/retry accessibility.

All fixture pumps and close drains use hard opportunity limits. No open-ended fixture loop remains in the P7b fill helpers.

## Exact residual census

The final scoped census reports zero production matches for:

- `.board_fill_snapshot(`;
- `BoardFillJob::new`;
- byte/JSON checkpoint encode or restore helpers;
- `BatchJobSession`;
- direct `.drive_step(`;
- `RevisionId(0)`;
- `run_on_worker`;
- JSON placement callbacks or `serde_json::Value` placement working-set authority;
- the former `handle_has_incident_edge(&handle.id)` whole traversal in capture;
- `Box::new(FillSessionNode)` uncredited registry backing.

The mounted action production slice additionally has zero `BoardHost`, `BoardFillSnapshotCapture`, `capture.step`, `FillWork::Capture`, `BoardFillJob::take_result`, `doc.snapshot.0.clone`, `BatchJobSession`, `RevisionId(0)`, `Generation(0)`, `Vec<Value>`, and `Vec<serde_json::Value>` matches. It has exactly one SnapshotRead acquisition, one shared process-pool authority, one fuel-one mount configuration, one terminal CommitCandidate match, and one CommitCandidate decoder.

The editor's live fill branch has zero whole document clone, `BoardHost`, `RefCell`, `BoardHost::default()`, host synchronization, and `puzzle2d_document_delta_operations` matches. Its direct typed mutation owner is the only document-output path before the ordinary command branch starts. The ordinary non-fill command branch retains its pre-existing BoardHost route outside the admitted fill continuation boundary.

The same branch has exactly zero `config.clone()`, `Puzzle2dConfigMutation::Snapshot`, `Vec<Value>`, and `BTreeMap` matches, exactly one `Puzzle2dFillRuntime::from_config(config)` projection read, and exactly one `Puzzle2dConfigMutation::Fill { runtime }` publication. The `Puzzle2dFillActionCtx` slice has zero `Puzzle2dConfig`, `Vec<Value>`, `BTreeMap`, and `String` matches. The fixed projection slice has zero `Vec`, `BTreeMap`, and `String` backing and exactly one fixed byte array owner.

The live `ArtifactBoardFillJob` capture slice has exactly one wrapper `capture_one` invocation per turn, seven retained `as_bytes().get(byte_cursor)` sites, 39 explicit ingress opportunities, and zero `for`, iterator, clone, dynamic string construction, or whole-record `push_node/push_handle/push_kind/push_rule` calls. Node geometry, handle geometry/visibility/connection, kind geometry/icon/template, rule source/target, and each fixed-page publication advance through distinct retained fields.

The fill core region has exactly eleven `try_push_owned` calls and exactly eleven `if let Err(exact_owner)` handbacks, with zero `.is_err()` discard. Placement has no detached reserve stage: destination credit and actual backing validation occur in the same Publish turn before either exact node/edge owner moves. Edge ID, edge kind, source, and target each have their own retained scalar stage.

There is exactly one production `root_cancel_token` construction: the retained session cancellation source created at operation start. Subsequent actions reuse that owner.

The remaining action `serde_json::json!` use builds only the fixed generation argument for the mounted continuation effect. Placement remains typed from the fixed worker page through `create_node`/`connect_handles`; no placement JSON is materialized or parsed.

## Static gates run

- Edition-2021 `rustfmt --check --config skip_children=true` on the Board core plus all eleven scoped Puzzle2d Rust files: GREEN.
- JSON schema parse with Bun `JSON.parse`: GREEN.
- Scoped `git diff --check` across all sixteen changed source/schema files: GREEN.
- Exact forbidden-shape census above: GREEN.
- Direct production-slice predicate reproduction plus all seventeen faithful mutations, including whole-config clone/snapshot/dynamic-runtime restorations: GREEN; all seventeen were rejected.
- Manual production-callee scan for loops/dynamic collections in the retained fill regions: GREEN; only the fixed 20-digit identifier formatter, fallibly pre-admitted four-item pages, the fixed 16-slot typed placement owner, and the final pre-sized schema-owned handle array remain. Each handle transfers in its own continuation.

No Cargo, Nx, Wasm, browser, or other build/runtime gate was run, as required while shared source packets overlap. Therefore this report makes no compiler, runtime, watchdog-execution, or cross-target acceptance claim.

## Deferred shared-verifier additions

The shared root verifier was explicitly outside this packet. A later serialized verifier packet should add faithful live-callee mutations that reject:

1. restoring `board_fill_snapshot`, byte checkpoint/restore, direct `BatchJobSession`/`drive_step`, revision zero, or JSON placement callback authority;
2. replacing `process_worker_pool`/`MountedWorkerJobSession` or changing fuel 1 / budget 7;
3. removing the retained handle-edge capture subcursor or restoring `handle_has_incident_edge` there;
4. removing same-owner `try_push_owned` handback, fallible four-item page admission, allocator-reported backing bytes, retained page count, or one-page close credit;
5. removing registry reservation/backing recovery, guard republish, generation matching, or terminal-empty assertions;
6. dropping typed checkpoint handback/adoption, one-handle apply, freshness checks, or terminal ACK/close;
7. removing the MAX/MAX+1, panic, stale, cancel, deadline, saturation, unclaimed terminal, worker-count parity, watchdog, or localized UI laws.
8. coalescing any retained capture/source/accept/apply stage, restoring a dynamic placement handle working set, or removing incremental JSON field close/backing accounting.
9. restoring a fill-branch `config.clone()`, whole `Snapshot` publication/inverse, or dynamic `Puzzle2dConfig`/`Vec<Value>`/map/string authority in `Puzzle2dFillActionCtx` or `Puzzle2dFillRuntime`.

## Handoff

The P7b source/static packet is ready for an independent Terra audit. Runtime/build acceptance remains deliberately deferred.

## Fourth independent RED remediation

The fourth independent re-audit found two remaining P0s: dynamic strings in the retained placement applier and a summary-only terminal candidate. Both are now closed in the live mounted path.

### Fixed retained placement ownership

`FillPlacementApplyCursor`, `FillPlacementHandleOwner`, `FillPlacementNodeOwner`, and `FillPlacementEdgeOwner` now retain only fixed `BoardFillText`, fixed scalar/enumeration fields, a fixed 16-slot handle array, and the exact typed placement/page owner. IDs, kinds, metadata, source/target IDs, and source edge kind advance through a shared retained byte cursor. One action continuation copies at most one character byte or one independent scalar/owner; fixed handle publication transfers one exact owner. Node icon presence and icon bytes have separate stages.

The final typed document mutation boundary remains intentionally same-turn and atomic. It first computes cumulative backing credits for both mutations, the schema handle array, every text buffer, the duplicated node label, shape, and optional icon. It then fallibly reserves the two mutation slots and exact handle capacity before constructing the schema-owned node and edge. Allocation refusal publishes neither mutation and leaves the authoritative fixed placement or terminal candidate retained for retry. The only three production `String` references in this region are these same-turn fallible output constructors; the retained owner slice has zero `String`, `Vec`, `BTreeMap`, `Puzzle2dNode`, or `Puzzle2dHandle` matches.

Checkpoint and terminal publication both pass an ephemeral borrowed `FillPlacementPublishView` over their authoritative fixed owners. The checkpoint path does not reassemble or copy a whole 10,406-byte commit placement or sixteen-slot handle array before publication. The publish-view slice itself owns only fixed references and scalars and has zero `String`, `Vec`, or `BTreeMap` matches.

### Full exact terminal candidate

`BoardFillResult` is no longer the whole terminal payload. The producer now commits a versioned fixed `BoardFillCommitCandidate` containing:

- accepted/stalled/search result scalars;
- optional full typed final placement;
- node ID and kind, geometry, shape, icon, and target-handle index;
- edge ID, exact source edge kind, source handle ID, and target handle ID;
- all sixteen possible fixed handle IDs/kinds/angles/radii plus exact handle count.

The canonical fixed codec is exactly 10,406 bytes and one admitted 16 KiB commit-output page at the current 256-byte text and 16-handle maxima. Encoding retains a field stage plus text byte cursor: header/result scalars, text lengths, every text byte, geometry scalar, handle field, writer construction, and page admission are distinct bounded worker opportunities. The exact payload decoder requires canonical magic/version, exact byte length, exact page count and page length, empty commit state, bounded handle count/target index, valid UTF-8, finite geometry, valid shape/presence tags, and the complete fixed placement.

The final accepted placement remains in `BoardFillJobState` instead of being emitted as a checkpoint. `StepOutcome::Complete(CommitCandidate)` therefore owns the exact full terminal projection. Both the initial pump and retained-outcome retry path decode that candidate and call the same pre-credited typed node-and-edge publication helper in the same action turn. Publication refusal retains the untouched exact `StepOutcome` page for retry. Only after publication succeeds, or a malformed terminal is faulted, does incremental ACK close one payload page and continue session/state retirement. No mutable `take_result` path exists.

The brush fixture now consumes the terminal placement as well as checkpoint placements, including exact edge kind, so count-one sessions witness the full candidate rather than losing their final placement from the oracle.

### Exact ownership and close table

| Owner | Refusal/stale/cancel behavior | Terminal close |
| --- | --- | --- |
| Fixed apply cursor | Keeps the exact source placement/page, partial fixed text, byte cursor, scalar stage, fixed handle array, node owner, and edge owner; no dynamic retained field can ordinary-drop. | Retires one fixed handle/owner or one placement item/page per action continuation. |
| Commit encoder | Keeps the canonical fixed 10,406-byte inline projection, field/text cursors, and output cursor inside the already admitted job backing. Deadline/cancel/stale prevents the next field or page opportunity. | Removes the fixed encoder as one owner; it has no separate heap backing. |
| Commit writer/page | Admission refusal remains in `RetainedJobPayloadWriter`; the next worker turn retries the exact page source. | Writer close releases one rejected/staged/committed page or writer owner per close grant. |
| Complete outcome | The exact full candidate page remains in `retained_outcome` until typed node+edge publication succeeds. | ACK closes one retained payload page per action continuation, then closes the mounted session and original job state incrementally. |
| Same-turn schema output | Cumulative backing is checked and both vector slots are reserved before node/edge construction or publication. | A failed construction emits no mutation; the authoritative fixed source remains retained. Successful construction transfers exactly the two typed mutation owners. |

### Fourth-remediation hostile laws

New permanent local laws reject:

1. injecting `String` into `FillPlacementEdgeOwner`;
2. injecting `String` into the complete mounted publish-view ownership slice;
3. replacing a retained per-byte edge-kind copy with a whole fixed-text assignment;
4. restoring whole fixed commit-placement and handle-array assembly in the checkpoint publish turn;
5. replacing the full candidate decoder with a summary decoder;
6. discarding `candidate.placement` while keeping only result scalars;
7. removing same-turn `try_reserve_exact(2)` destination credit;
8. injecting `String` into `BoardFillCommitPlacement`;
9. restoring a 13-byte `BOARD_FILL_COMMIT_BYTES` terminal;
10. replacing one-byte terminal text encoding with whole-slot `copy_from_slice`.

An executable MAX/MAX+1 law additionally advances a 256-byte fixed placement label one byte per cursor call, proves each call grows the destination by exactly one byte, and rejects 257 bytes. The Board law fixes the full candidate layout at 10,406 bytes and exactly one admitted page.

### Fourth-remediation residual census and gates

The post-format production census is:

- retained placement-owner slice: `String=0`, `Vec<` `=0`, `BTreeMap=0`, `Puzzle2dNode=0`, `Puzzle2dHandle=0`, fixed `BoardFillText=11`;
- borrowed publish-view slice: `String=0`, `Vec<` `=0`, `BTreeMap=0`, exact commit-view binding `=1`, fixed cursor-view authority matches `=2`;
- retained apply implementation: exactly nine `copy_fill_text_one` call sites;
- retained apply implementation: whole commit-placement assembly `=0`, whole handle-array reassembly `=0`;
- terminal codec slice: `String=0`, `Vec<` `=0`, `BTreeMap=0`, `single_page()=0`, 13-byte terminal constants `=0`, full placement definitions `=1`, exact edge-kind field `=1`, fixed text encoder `=1`;
- mounted action production: exact full-candidate decoder `=1`, initial/retry terminal publication calls `=2`, `BoardFillJob::take_result=0`, summary-result decoder `=0`;
- fixed-page core: `try_push_owned=11`, discarded `.is_err()` handbacks `=0`;
- early fill branch preservation: `config.clone()=0`, whole config snapshot mutation `=0`, `BoardHost=0`, whole document clone `=0`.

Current source/static gates:

- edition-2021 rustfmt and a subsequent `--check --config skip_children=true` across the Board file plus all eleven declared Puzzle2d Rust files: GREEN;
- Bun JSON schema parse: GREEN;
- four new live source-predicate baselines plus all ten fourth-remediation mutations: GREEN; every mutation was rejected;
- six preserved mounted capture/dispatch/runtime/ingress/handback predicate baselines: GREEN;
- scoped `git diff HEAD --check` across the sixteen source/schema files and this report: GREEN;
- exact residual census above: GREEN.

No Cargo, Nx, Wasm, browser, compiler, runtime, or watchdog gate was run. P7b remains source/static audit-ready, not self-accepted.
