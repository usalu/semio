# Jack Query Runtime Bounds

## Retained execution

- `QueryExecutionPreparation` clones snapshot metadata through `JackSnapshotCloneAuthority`, then admits and clones one node or edge per turn. Node and edge admission walks at most 128 nested property items to depth 16 and rejects any entity whose owned clone exceeds the 4,096-byte turn grant before calling `clone`.
- Manifest property cloning now includes nested `ValueType::List` boxes and schema strings in its byte admission. `semio-framework-graph::manifest::ValueType` is explicitly re-exported because it is the public type of `PropertyDef::value_type`; Jack does not add a neural-engine dependency.
- `QueryExecution` mutates its retained graph directly. Node deletion scans or removes one edge per turn. Replaced and removed values enter `JackMutationRetirementFactory` before evaluation continues.
- Table returns advance one column, binding/item inspection, row, or cell per turn. The result counter includes fixed result structure, columns, rows, delimiters, JSON escapes, values, and graph metadata. Its recursive value walk is item/depth bounded. Graph nodes and edges are admitted and hashed without constructing a JSON string, then cloned only after the output limit accepts them.
- The 1 MiB check fails before a result can be returned. A native regression builds the neutral 300-row by 3,900-byte-cell case, establishes that its cells alone encode above 1 MiB, observes `query result exceeds its output admission`, and retires the rejected execution through one-item/4 KiB close steps.

## Checkpoints

- Checkpoint format `JQR2` is 32 bytes and stores progress, raw operation id, and raw generation. Restore compares the raw tuple, so the known legacy XOR collision `(501, 12)` versus `(4_194_805, 13)` is rejected even though both values still produce the same framework workspace hash.
- The replay ceiling is derived from the 4,096-byte query admission and the three 16,384 binding/entity scan dimensions, plus one million preparation/result/retirement turns. A native regression restores progress `1,000,001` and rejects the first value above the derived maximum.
- Restore continues to create an empty workspace and deterministically replays the recorded number of retained steps. No checkpoint claims to serialize or restore an in-memory graph cursor.

## Shared snapshot ownership

- Ordinary `JackSnapshotCloneAuthority` clones retain the exact local `Arc<JackWorkingScene>`; metadata-only clones used by query preparation do not retain the scene.
- Snapshot retirement removes its local owner first. A shared scene drops only that `Arc` reference in constant work. The unique last owner transitions to the existing scene retirement cursor and drains one node or edge subtree at a time.
- Native tests cover shared-source-live retirement, unique last-owner draining, cancellation of a partially prepared query while its source remains readable, and complete retirement after oversized node/edge admission failures.

## Ownership publication evidence

- `JackQueryWork::complete` emits query graph operations in the artifact lane, the authored `SetQuery` mutation in the app config lane, and exactly one `ReplaceQueryResult` in the app transient lane. It emits no window transient value.
- Editor integration tests inspect native typed-operation receipts: `query_ownership_runtime_publishes_transient_result_without_document_edit` requires one app-transient receipt, unchanged document content, one transient generation increment, a rendered table, and complete app retirement. `query_ownership_window_carets_and_query_publish_independently` requires one app receipt and two concrete-window receipts with independent generations.

## Neutral and native validation

- `bun nx run workspace:test-jack-query-oracle` passed on 2026-09-09. SQLite matched six query result/mutation cases and the two concrete-window owners. The retained oracle also reproduced the old XOR collision while storing both raw owner tuples, admitted legal replay progress above one million, rejected an 8,192-byte entity against the 4,096-byte grant, and measured the 300 by 3,900-byte cell array above 1 MiB.
- `CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/cargo-trinity bun nx run workspace:test-jack-query-ownership` compiled and ran 11 tests in exec session `64175`. Checkpoint identity, long replay, output admission, shared/unique retirement, cancellation, artifact contracts, and the observed neutral executor cases passed. The first oversized-entity test run used positional fixture lookup and failed because `child` preceded `root`; it now finds the exact `root` and `e1` owners. The two editor integration tests aborted before app creation on the separately owned generated catalog classification for `setViewport` (`generated_migrated=false`), followed by the test harness disposer panic. A warm native rerun after those two corrections remains pending and must be recorded before ticket closure.

## Exact files

- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs`
