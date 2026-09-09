# Terra Final Audit: Jack Retained Query Runtime

Scope: source-only review of the eight runtime, command, fixture, and test files named in `jack-query-runtime-bounds.md`. No source changes or native build were run because the shared all-artifact validation is active.

## Result

The retained preparation, direct graph mutation, one-item node-deletion scan, shared-`Arc` retirement, and transient-result handoff are implemented as described. Six correctness issues remain and should block a final retained-query sign-off. Two were confirmed by the completed native component run after this source review began.

### P1 — Preparation reports complete before it has retired its query owner

The native `query_ownership_cancelled_preparation_closes_while_source_scene_remains_live` test fails its final `terminal_is_empty` assertion. In [`🪜️execution/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs) line 102, the outer preparation returns `self.metadata.close_step(...)` directly. On the final clone-retirement call, the nested clone correctly returns `Complete`, but the outer `QueryExecutionPreparation` still owns `query: Some(_)` and has not set `terminal`.

The existing test loop treats that forwarded `Complete` as its terminal signal, then finds the preparation nonempty. Convert a nested `Complete` into outer `Pending` after verifying that `self.metadata.terminal_is_empty()`, as the adjacent `metadata_retirement` branch already does. The next outer close turns can then retire `query` and set the outer terminal state. This is a false completion signal and must not reach the interactive-job close protocol.

### P1 — An empty return projection produces one empty row

The completed native neutral-case run reports the `CREATE (n:Piece)` result as `[[]]` where the neutral fixture expects `[]`. In [`🪜️execution/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs) lines 261–273, `ReturnExecution::Table` appends `Vec::new()` when `rows.len() == binding` before it checks `items.is_empty()`. A binding with an empty projection therefore materializes an empty row.

Check `items.is_empty()` before allocating/appending a row. That preserves the fixture's zero-row representation for a zero-column mutation result and avoids allocating a semantic placeholder that is not part of the query output.

### P1 — Checkpoints do not bind uniquely to an operation and generation

[`▶️run-query/🧵️job/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs) computes the only owner field as an XOR of the tool, `operation_id.rotate_left(19)`, and `generation.rotate_left(41)` at lines 87–90, then writes and checks only that 64-bit value at lines 159–168. XOR plus rotations is not injective for the two 64-bit inputs.

Reproduction: for the same `runQuery` tool, a checkpoint made for operation `501`, generation `12` has the same stored identity as operation `4_194_805` (`501 ^ 2^22`), generation `13` (`12 ^ 1`), because `rotate_left(2^22, 19) == rotate_left(1, 41) == 2^41`. A `JQR1` record from the first owner therefore passes `restore` for the second owner and replays against the wrong operation/generation pair. Store both raw `u64` values in the checkpoint and compare them exactly; the factory's 64-byte checkpoint limit leaves room.

No listed test exercises a successful restore, a wrong-owner rejection, or this collision class.

### P1 — The replay maximum is far below the legal retained-query work bound

[`▶️run-query/🧵️job/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs) writes every `progress` value into a checkpoint (lines 156–162), but restore rejects a value over `QUERY_REPLAY_MAXIMUM_STEPS`, fixed at one million (lines 17 and 166–169). The command nevertheless admits 16,384 nodes and 16,384 edges (lines 114–117). `PatternExecution` advances one node/edge candidate per turn (the nested scans at `🪜️execution/🦀️.rs` lines 141–178), so an admitted edge match can require roughly 268 million turns before it completes.

Reproduction: give the command 16,384 `Piece` nodes and 16,384 `Connection` edges which do not match the current node. `MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name` scans all edge candidates for each node without reaching the match-row cap. A checkpoint written after progress `1,000,001` is accepted by `checkpoint`, then rejected by `restore` for the same operation. Derive the maximum from the actual admission bound or reject checkpoint creation before emitting a non-restorable checkpoint. Add a legal long-scan checkpoint/replay test.

### P1 — The 1 MiB output admission undercounts the emitted table and admits an oversized transient result

[`🪜️execution/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs) records only the serialized payload length of columns, values, nodes, and edges (lines 205–208, 216–219, 235–249, and 261–273). It excludes row/array delimiters, commas, result fields, and graph snapshot metadata. It also serializes a node or edge before checking the admission (lines 237–238 and 246–247), so one oversized entity has already allocated its complete JSON string when it is rejected.

Reproduction: admit a snapshot with 16,384 `Piece` nodes (the exact entity limit) and run a sub-4 KiB query with 16 missing-property columns, such as `MATCH (a:Piece) RETURN a.p0, …, a.p15`. The table path produces 262,144 `null` cells. Its counter is exactly `262,144 * 4 = 1,048,576`, so it accepts the result because the check is `>` rather than `>=`. The rows alone encode to at least `16,384 * 81 = 1,327,104` bytes (`[null,...,null]` per row), before result and column JSON. The emitted transient result consequently exceeds the documented 1 MiB admission by more than 25%.

The output budget must account for the actual emitted representation before constructing it, including table structure and graph metadata, and must not first serialize an arbitrarily large entity in one retained turn.

### P1 — Preparation’s node and edge copies are not bounded by its byte grant

[`🪜️execution/🦀️.rs`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs) accepts `maximum_bytes` in `QueryExecutionPreparation::step` and applies it only to metadata cloning (lines 35–40). It then clones a complete source `Node` or `Edge`, including nested property values and ports, in one turn (lines 70–82). The command admission only limits entity and manifest-member counts; it has no node/edge/property byte bound (`▶️run-query/🧵️job/🦀️.rs` lines 114–117).

Reproduction: use a snapshot with one admissible `Piece` node whose nested data property contains a multi-megabyte string, then start a retained query. `extent` accepts it because the scene has one node, but the preparation node turn executes `node.clone()` and allocates/copies the entire payload despite the job passing the 4,096-byte maximum. The operation is item-incremental but not budget-bounded. Stage node/edge fields and property values through a retained cursor, or make an explicit, enforced byte admission before the clone.

### P2 — The SQLite oracle does not cover the retained-runtime properties it is cited for

[`🟦️.ts`](../../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts) independently checks six fixed fixture query answers against SQLite (lines 7–24). The Rust test separately checks the resumable executor against the same fixture and the synchronous in-repo executor (lines 179–219 of `🧪️tests/🔬️unit/🦀️.rs`). That is useful semantic coverage for those small queries, but the oracle never invokes `QueryExecution`, `QueryExecutionPreparation`, `JackQueryWork`, checkpoint restore, close, or transient publication.

Its cancellation section only inserts seven rows into a separate SQLite `progress` table and then asserts that its separately inserted source row remains live (TypeScript lines 38–50). Its interleaving section only inserts selection rows and compares fixture constants (lines 26–37). Neither can detect a Rust cancellation/retirement error, a replay-owner collision, an output-cap bypass, or result leakage into document mutations. Add direct retained-command tests for checkpoint replay/rejection, byte-exact output admission, cancellation with a shared source, and inspection of the emitted `CompleteWithEphemeral` lanes; keep SQLite for the independently expressed query semantics.

## Confirmed Source Properties

- Preparation uses `JackSnapshotCloneAuthority::metadata_only()` and advances metadata before copying one node or edge per call (`🪜️execution/🦀️.rs` lines 31–87). It does not invoke the old complete-fixture graph rebuild path.
- Execution mutates the retained `Graph` directly. Node deletion advances past one edge and removes at most one incident edge in an evaluation call; removed/replaced values are placed in `JackMutationRetirementFactory` first (`🪜️execution/🦀️.rs` lines 315–367).
- A shared scene `Arc` is released immediately when `Arc::try_unwrap` fails rather than waiting for the live peer (`🛜️wire-runtime/🦀️.rs` lines 354–370). The wire-runtime unit test creates a shared clone, closes it, and reads the original source afterward (unit-test lines 159–172). The preparation-cancellation test likewise retains and reads the source scene after close (executor unit-test lines 235–261).
- The command’s successful completion puts `ReplaceQueryResult` solely in `EphemeralEmit.transient`; document mutations are the query's graph operations and the authored `SetQuery` config mutation (`▶️run-query/🧵️job/🦀️.rs` lines 102–107). This source trace keeps computed output out of the emitted document/config lanes.
- Close respects an item grant by intentionally doing no more than one child retirement unit and reports the child result. Its query-source release is truthful for the documented 4,096-byte grant: it blocks, rather than claiming release, when the supplied byte grant cannot cover the admitted source (`▶️run-query/🧵️job/🦀️.rs` lines 172–197).

## Validation Status

No commands were launched by this source-only audit. The ticket's existing Bun oracle result is not repeated here. During the review, the coordinator reported that the completed component-native run failed four of seven `query_ownership` cases: the two failures above and two separately scoped app-definition classification failures for `deleteSelection`, `formatDocument`, `reorganize`, `setActiveExample`, and `setFixtureJson`. The coordinator applied bounded source fixes for the two native runtime failures after that result; this report preserves the before-fix evidence and does not claim a post-fix native pass. Native validation remains owned by the active all-artifact run.
