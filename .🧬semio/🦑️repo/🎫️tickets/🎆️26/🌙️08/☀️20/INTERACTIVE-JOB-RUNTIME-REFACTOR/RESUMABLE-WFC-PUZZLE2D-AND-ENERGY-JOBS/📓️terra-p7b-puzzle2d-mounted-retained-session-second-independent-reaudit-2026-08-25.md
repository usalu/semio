# Terra Independent P7b Mounted Retained Fill Re-audit

Date: 2026-08-25  
Auditor: Terra independent read-only source/static audit  
Verdict: **RED — do not accept P7b.**

## Scope and method

I reread the P7 master plan, the P7b contract, the earlier Terra RED, and the updated Sol report. I then inspected the current live Puzzle2d editor/action/UI/fixture sources, the Board fill callee bodies, and the framework SnapshotRead implementation and caller. This is a new report and does not replace the prior RED.

No production source, shared verifier, Cargo, Nx, Wasm, browser, build, or runtime test was run. The source-only checks recorded below were run on the current tree.

## Decision

The three previous primary route failures are closed: the real fill branch now precedes the old document clone/sync/delta route, capture runs inside an ArtifactBoardFillJob submitted through the shared InteractiveNative process pool, and terminal results are decoded from the retained CommitCandidate before that outcome is incrementally ACKed. All eleven live fixed-page insertion call sites now bind the returned owner.

P7b is still RED. Every live fill continuation constructs and ordinarily drops a populated BoardHost on the UI/action path. That is specifically disallowed by the new requirement that the action only reserve, enqueue, or poll retained work; it also bypasses the one-owner/page close discipline. The worker capture is additionally coarser than the required one field/edge per fuel grant.

## P0 — Every fill action constructs and deep-drops a populated BoardHost outside the retained worker/close path

The early branch avoids document materialization but it is not action-light:

1. Puzzle2dPlayApp::handle treats setFillCount, begin, step, adopt, cancel, retry, discard, and clear as fill actions at editor component lines 936-949. At lines 1014-1037, every one of those actions creates a scene, calls BoardHost::default, wraps it in RefCell, and then dispatches the action. The temporary host has no retained owner, no close cursor, and is dropped when that branch returns.
2. This is not an empty POD shell. BoardHost contains BTreeMap/BTreeSet/Vec/HashMap authorities, a BoardEventQueue, and a BrushCandidatePage at Board normal component lines 1799-1882. Its Default body constructs builtin edge tips, BoardEventQueue::default, and BrushCandidatePage::default at lines 3033-3111.
3. BoardEventQueue::default allocates a boxed fixed slot array at Board normal component lines 1651-1654. BrushCandidatePage::default allocates both its boxed byte backing and boxed entry backing at lines 242-245. builtin_edge_tips constructs and inserts five owned BTreeMap records at Board directed component lines 273-280.
4. BoardHost has no Drop implementation; only the separate BoardHostRetirement type has one. Therefore this production branch lets Rust's ordinary destructor recursively release the populated host and all of those owners after every queued/poll/adopt/cancel/discard action. It never enters BoardHostRetirement or a bounded close grant.

This is a direct counterexample to “action only reserves/enqueues/polls” and to the required one-owner/page close / no ordinary populated Drop rules. It remains even though the host is no longer synchronized from the ArtifactView. The existing editor source predicate only rejects reintroducing doc.snapshot clone, sync_host_fixture_content, or puzzle2d_document_delta_operations; it permits this already-present BoardHost::default authority, so its three route mutations cannot prove the required action surface.

## P0 — Artifact capture still performs multiple fields and owned text copies for one fuel grant

ArtifactBoardFillJob::step checks cancellation, operation, canonical SnapshotRead authority, and deadline, then calls capture_one and consumes exactly one fuel unit at set-fill-count component lines 290-315. capture_one, however, does not retain a field cursor for several input records:

1. capture_node_one reads id, x, y, scale, shape, width/height or radius, computes bounds, and calls ingress.push_node in a single grant at lines 110-129. push_node creates BoardFillText from the ID and publishes the node in that same continuation at Board component lines 5375-5387.
2. Once its one-edge-at-a-time scan ends, capture_handle_one reads the handle ID, node/handle/wire/edge kinds, position, angle, size, visibility, connection flag, and calls push_handle in one grant at action lines 132-186. push_handle copies five independent BoardFillText fields and publishes the handle in that same grant at Board lines 5391-5413.
3. capture_kind_one begins a kind by copying ID and optional icon together with shape and geometry at action lines 189-225; begin_kind creates those retained text owners together at Board lines 5417-5431. capture_rule_one copies both source and target text fields before publishing the rule at action lines 228-249 and Board lines 5470-5490.

The contract requires one admitted field or fixed collection slot per grant, and the re-audit request explicitly requires one field/edge per fuel. These are multiple independent input fields and text-owner constructions after only one worker opportunity. They must be represented by retained field cursors, each rechecking cancellation/deadline/freshness before its own transfer, rather than treating an entire JSON record as a unit.

## Confirmed closures and preserved green findings

- The live fill branch is before the old `doc.snapshot.0.clone`, BoardHost synchronization, parse_fixture_v1, and whole document delta path: editor component lines 1014-1037 precede the ordinary branch at 1038-1105. The branch source contains none of those three whole-work calls.
- pending_effects obtains the one matching SnapshotRead lease and creates MountedWorkerJobSession with fuel_per_step 1 and step_budget_ms 7 at action lines 930-962. ArtifactBoardFillJob owns the lease, checks canonical authority at lines 290-300, and returns it through a retained generation witness during incremental close at lines 322-364. The store witness and maintenance authority are real: store component lines 231-285 and 12923-12930.
- The real worker pump uses the shared InteractiveNative process pool and MountedWorkerJobSession::pump_one at action lines 925-928 and 1185-1285. No action-file production BoardFillJob::take_result call remains.
- The terminal branch matches StepOutcome::Complete(candidate), decodes BoardFillResult from that candidate, retains the outcome, and closes/ACKs it one page at a time before closing work at action lines 1227-1259 and 1322-1333. BoardFillJob encodes the result as the CommitCandidate output at Board lines 6257-6282.
- Paged placement applies typed node and edge mutations, reserves its two destination mutations before moving either owner, and carries the source edge kind through EdgeKind to connect_handles at action lines 559-587. The apply close walks individual handles, fields, and backing owners at lines 601-657.
- The fixed eight-slot registry reserves before node allocation, publishes only after initialization, generation-matches take, republishs a checked-out guard in Drop, and returns a pre-publication reservation in Drop at action lines 796-905. Source fixtures include MAX/MAX+1, pre-publication panic, lost guard, and u64::MAX non-aliasing laws at lines 1502-1551.
- The retained fill region has exactly eleven current try_push_owned uses. All bind if let Err(owner) and restore that owner, including the compatibility path at Board lines 5886-5891 and 5916-5924. There is no production compatibility .is_err discard.
- The UI has live localized progress/cancel/retry/fault controls at fill tool lines 20-85, with EN/DE terminology at terminology lines 45-52 and an explicit German accessibility source fixture at fill tool lines 114-140.

## Source mutations and census

I reproduced the seven current in-memory source mutations without compiling or changing a file:

| Mutation family | Result |
| --- | --- |
| Restore whole ArtifactView clone | rejected by editor predicate |
| Restore BoardHost sync/rebuild | rejected by editor predicate |
| Restore whole document delta | rejected by editor predicate |
| Remove ArtifactBoardFillJob worker implementation | rejected by action predicate |
| Restore mutable take_result terminal reread | rejected by action predicate |
| Remove same-turn mutation destination credit | rejected by action predicate |
| Restore compatibility Err(candidate) discard | rejected by Board predicate |

All seven existing mutations are rejected. They are not sufficient to accept the packet because none removes or forbids the live temporary BoardHost::default, and none tests the missing Artifact capture field cursors. The independent census found: 11 ingress-to-job try_push_owned calls; zero old whole clone/sync/delta calls in the early editor branch; zero production action BoardFillJob::take_result calls; and one BoardHost::default call in that same early branch.

## Scoped static gates

| Gate | Result |
| --- | --- |
| rustfmt --edition 2021 --check --config skip_children=true on Board and the eleven scoped Puzzle2d Rust sources | PASS |
| Bun JSON.parse of the Puzzle2d fill config schema | PASS |
| Scoped git diff --check across Board/Puzzle2d P7b sources | PASS |
| Seven faithful in-memory source mutations | PASS — all rejected, but incomplete for the two P0s above |
| Live-callee ownership/census | FAIL — early branch creates BoardHost::default; capture coalesces fields |

## Required closure

1. Remove BoardHost construction from the complete fill-action branch. Give the fill action context only the minimal retained-runtime/effect/mutation authorities it actually uses, and make setFillCount's tool selection a bounded direct UI/config operation without a host. No continuation may create, own, or ordinary-drop BoardHost or its child backings.
2. Split ArtifactBoardFillJob capture into retained field cursors. Node ID, each geometry/visibility scalar, handle text/kind/edge field, kind/icon/template field, and rule source/target must be distinct admitted opportunities; retain every partial owner and recheck cancellation, deadline, operation/generation, and SnapshotRead authority before each transfer and snapshot publication.
3. Add faithful production-slice mutations/laws that fail if BoardHost::default or any BoardHost owner returns to the fill branch, and if any multi-field capture helper is restored. Cover the exact lease return and close path under cancellation, stale render authority, worker refusal, unclaimed terminal, guard panic, and interrupted close.
4. Rerun source audit after those changes. Compiler, runtime, worker-count parity, saturation/panic/close stress, allocation, native/Wasm, and watchdog execution remain deferred and make no acceptance claim here.
