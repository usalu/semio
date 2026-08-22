# P7h Independent WFC Checkpoint Final Audit

Date: 2026-08-22  
Scope: Isolated P7g WFC checkpoint-boundary repair, with read-only preservation checks of the
cold-relay cleanup ownership, hot-shard cancellation, ActionBus freshness route, and inference
bridge.  
Method: Fresh static/source inspection only. Read root and applicable source instructions, all of
`p7a-wfc-job.md`, `p7f-independent-wfc-cancel-final-audit.md`, and
`p7g-independent-wfc-cleanup-final-audit.md`, including their dispositions. No Cargo, Bun, build,
test, runtime, Wasm, cache/target deletion, ticket-status/metadata, git, or production-source
operation was performed.

## Verdict: PASS

The P7g checkpoint P0 is repaired in the current source. Static inspection finds one derived
checkpoint fixed-header contract, one shared checked aggregate byte calculation, real exact-maximum
serialization and phased restoration source coverage, and the requested before-allocation admission
checks. No stale standalone checkpoint `168` or `176` literal remains in the WFC Rust source.

This is a source/static PASS only. It does not claim compilation, test execution, timing,
allocation, native, release, or Wasm acceptance.

## Checkpoint Contract And Bounds

| Requirement | Current source evidence | Result |
| --- | --- | --- |
| One fixed-header source of truth | `CHECKPOINT_FIXED_HEADER_BYTES` is `CHECKPOINT_MAGIC.len() + CHECKPOINT_HEADER_U64_FIELDS * size_of::<u64>()`. The grouped field counts are identity 8, RNG 4, progress 3, and counts 5: exactly 20 `u64` fields; with the eight-byte magic this derives 168 bytes without a standalone literal (`wfc job:18-28`). | PASS |
| Encoder count and decoder order | `checkpoint_one` declares `let fields: [u64; CHECKPOINT_HEADER_U64_FIELDS]`, so the compiler enforces all 20 encoded fields (`:991-1016`). `decode_header` reads the same identity, terminal, four RNG, three progress, and five count fields in that order, then requires the derived cursor position (`:1270-1302`). | PASS |
| Shared checked sizing | `CheckpointCounts::checked_bytes` derives domain words and every variable section using checked arithmetic (`:310-331`). `CheckpointBuild::new` uses it before capacity reservation (`:353-366`); `WfcRestore::decode_header` uses the same function for exact input-length validation (`:1301-1316`). | PASS |
| Derived write guards | Header, domain word, trail, decision, and observed writes respectively guard the derived fixed-header, `u64`, trail-entry, decision-entry, and observed-entry sizes before appending (`:991-1069`). The entry constants match the serialized fields: 8, 48, and 8 bytes for trail, decision, and observed entries (`:26-28`). | PASS |
| No stale header literals | A direct static scan of the WFC Rust source found no `168` or `176` literal. The only fixed-header references resolve to `CHECKPOINT_FIXED_HEADER_BYTES`. | PASS |

## Exact Maximum, Rejection, And Decoder Coverage

| Requirement | Current source evidence | Result |
| --- | --- | --- |
| Real exact-maximum producer | `prepare_maximum_checkpoint_state` completes a one-node job, derives the remaining observed-entry count from `MAX_CHECKPOINT_BYTES - CheckpointCounts::checked_bytes()`, verifies the resulting count equals the maximum, and never patches serialized bytes (`:1709-1722`). `maximum_checkpoint` invokes production `begin_checkpoint` and incremental `checkpoint_one`, asserting the emitted vector length is exactly `MAX_CHECKPOINT_BYTES` (`:1725-1735`). | PASS |
| Phased maximum restore to completion | `maximum_checkpoint_restore_is_bounded_and_cancellable_in_every_phase` admits that emitted vector, statically covers cancellation at Header, Domains, Trail, Decisions, Observed, Verify, Rebuild, and Complete, drives through `StepOutcome::Complete`, and takes the restored job (`:1911-1940`). | PASS |
| Raw and typed maximum-plus-one admission | `WfcRestore::new` rejects `bytes.len() > MAX_CHECKPOINT_BYTES` before initializing decoder state (`:1216-1253`); its test supplies `MAX_CHECKPOINT_BYTES + 1` and requires `wfc-checkpoint-admission-exceeded` (`:1943-1947`). `CheckpointBuild::new` computes and compares capacity before `try_reserve_exact` (`:354-364`); the allocation fixture adds one observed item and requires typed `wfc-checkpoint-admission-exceeded` (`:1973-1990`). | PASS |
| Meaningful header-only and overflow coverage | The zero-domain terminal checkpoint must equal the derived fixed-header size and restores through the real restore path (`:1949-1956`). The overflow fixture modifies the final observed-count header field, starts the real incremental restore, and requires `wfc-checkpoint-capacity`; direct maximum count arithmetic must also return `None` (`:1959-1969`). | PASS |

## Preservation Checks

| Surface | Current static evidence | Result |
| --- | --- | --- |
| CleanupPending relay | Mounted leases default to `CleanupPending`; their Drop publishes that state before the semaphore permit is released (`host:2812-2908`, `:2973-3051`). Cleanup uses an exclusive cleanup lease and only success restores `Available` (`:2837-2855`, `:3056-3083`). The three barrier regressions hold the post-release/pre-cleanup interval and require rejection/no re-entry (`:3904-3987`). | PASS |
| Hot-shard cancellation | `Effect::CancelJob` removes running-job records only after `Ok(())`; failure unregisters the actor and publishes `ShardOutcome::Fault` (`shard:606-627`). Actor-level `Payload::Cancel` walks the stable `BTreeSet`, retains the first fault, and unregisters regardless (`:736-758`). | PASS |
| ActionBus/freshness | Procedural plugin construction registers the Assembly factory on `ActionBus::production` (`procedural:139-147`). The inference bridge reconstructs the exact key/schema for dispatch (`infer bridge:171-208`), while the host removes and validates live revision/generation authority immediately before exposing a result (`host:4975-4995`). | PASS |
| Fixed bridge bounds | Checkpoint/commit storage remains a fixed two-slot array; aggregate bytes use `checked_add`, with explicit item and byte saturation (`infer bridge:69-127`). | PASS |

## Explicitly Unrun Gates

No executable result is claimed for this current tree. The following remain unrun in this audit lane:

- debug and release compilation plus focused/full relay, hot-shard, Assembly, WFC, and inference-bridge tests, including the exact-maximum, header-only, overflow, MAX+1, cleanup-race, retained-waker, and cancellation cases;
- allocation-pressure, p99/max watchdog, race, and one-worker-progress runtime measurements;
- public-factory replay at 1/2/4/default worker counts and mounted `semio.infer` freshness/document-close integration;
- procedural native development, strict `-D warnings`, and release gates;
- `wasm32-unknown-unknown` and `wasm32-wasip2` build/runtime gates.

Historical command records in P7 notes were read for context only and were not accepted as current-audit execution evidence.
