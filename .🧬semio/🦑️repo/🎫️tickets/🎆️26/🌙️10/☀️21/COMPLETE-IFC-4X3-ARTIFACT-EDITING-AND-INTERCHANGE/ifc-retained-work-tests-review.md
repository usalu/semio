# Actual Retained Work Tests Static Review

Read-only review of details/work_tests.rs, work.rs mount and both version wrapper test aliases. No native compile/execution claim.

## Concrete Compile Finding

Initial inputs helper used let config=NoConfig, but framework NoConfig is an empty braced struct, not a unit constructor (plugin/📜️declaration/👁️view/🕳️empty-lanes/🦀️.rs:14). Reported directly to High/main; use NoConfig{} or NoConfig::default(). Fresh source reads should confirm correction before native execution.

## API and Branch Review

super::super::super correctly reaches wrapper aliases from wrapper.shared.work.retained_work_tests. IFC4 alias names and command EditSnapshot match actual editor. IFC4x3 wrapper follows its own actual editor type; shared snapshot/mutation identity remains same. AppOperationContext field types match authored inputs; HistoryView::empty is real public constructor; ArtifactCommandSnapshotRead::unavailable safely expresses absent Store custody for pure work. Captured Arc snapshot_owner permits real retained source admission and repeated clone steps. No obvious borrow escape found in inputs closure or direct pending emission match.

prepare drives actual source/clone states and waits until entity/value exists; main final step is therefore the real custom retained branch, not merely planner. For current single string payload, normalization charges1tick and native argument edit charges1tick, so fuel2 reaches pending after value.take and before publication validator. Subsequent adequate-fuel call resumes stored mutation, exercising the earlier recopy/panic risk. Malformed1000arg shrink now reaches actual cap validator. Success mutation/inverse loop uses sequential base and reverse undo insertion, appropriately proving primitive reversibility independent of originalbase batch precheck.

## Limits of Original Caller and Close Evidence

turn currently creates a fresh root cancellation token and StepContext for every preparation/final/close call. It therefore exercises actual work and close methods, including cancellation of that invocation, but does not preserve or prove the same initiating physical caller/cancellation node across turns. Use existing OriginalWorkFixture for that stronger claim; this was sent to High/main. The repeated close loop proves logical work.terminal_is_empty only if executed; syntax parsing does not establish it. Arc strongcount assertion in pending test is a useful actual owner release assertion once native execution is available. These tests stop before factory wire/preflight and mounted Store publication, so they do not prove history gesture, stale revision or durable publication behavior.

No source changes by audit.

## Fresh Fixture Correction

Fresh work_tests.rs now constructs NoConfig{} and uses one OriginalWorkFixture per Work through source/copy/final/close, with matching operation716/generation3. work_turn narrows original fuel, cancellation targets caller.cancel(), close uses caller.close(), and finish retires source fixture roots. Previous constructor and cross-turn fresh-root findings are resolved by source inspection. This still requires native execution and remains a pure custom-work test before mounted Store publication.
