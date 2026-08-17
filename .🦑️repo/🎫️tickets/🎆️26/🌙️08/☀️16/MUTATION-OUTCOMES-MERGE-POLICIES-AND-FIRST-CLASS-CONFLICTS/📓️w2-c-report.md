# Lane 2-C (TS host + kernel) — C8/C9 report

## Types (kernel `🎠️kernel/🟦️component.ts` region `🔖️MergeOutcome`, ↔ Rust)
`Severity` fixed (`"info"|"warning"|"error"|"fatal"`, was stale `"fatal"|"error"|"warning"|"hint"` — C1's `Hint`→`Info` TS mirror site). `MergePolicy`=`"LaissezFaire"|"Normal"|"Vigilant"` (no rename_all, bare variant names) + `DEFAULT_MERGE_POLICY="Normal"` (boot default) + `mergePolicyAsU8/FromU8`. `MutationMessage{level,code,message,target?,opIndex?}`, `ConflictId=string`, `ConflictKind` (tagged, `edit_ids` stays snake_case), `ConflictStatus`, `ConflictResolution`+ordinal helpers, `Conflict`, `EditMessages`, `DispatchReport{policy,worst,messages}`, `MergeReport{policy,accepted,insertionIndex,replayed,worst,conflict}` — camelCase per Rust's `#[serde(rename_all="camelCase")]` structs.

## AppChannelClient (`💻️os/🟦️component.ts`)
`APP_CHANNEL_VERSION` 10→11. Added `setMergePolicy/resolveConflict/readConflicts` (region `🔖️Merge`). `MergeReport`/`Conflicts` frames pass through `drain()`/`exchangeOne()`'s existing generic `AppFrameValue[]` return (no filtering) — surfaced via new decode helpers `decodeDispatchReportFromWire`, `decodeMergeReportFromWire`, `decodeConflictsFromWire`, and `faultMessages(reportBytes, decodePackValue)` (typed `MutationMessage[]` out of `Error.report`), all following the existing `decodeFaultFromWire` DI-parameter convention. Fixed 1-C's flagged mock at the `command()` test (missing `messages: []`) and the `hello()` seq-11 expectation.

## Parity test
Added to `AppChannelClient` describe block: "setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte" (drives the client's own methods to hit seq 5/6/7, asserts against `app-command-merge.json`), plus a frame-surfacing test and a decode-helper round-trip test against the frozen TS shapes.

## Test results
Direct `bunx vitest run --config 🧪️vitest.config.ts` (from `📦️packages/🟦️typescript`) — **nx path starved** (`bun nx run @semio-tech/framework-os:test` looped on repeated `nx run` echoes for 2.5+ min with zero test output, same symptom 1-C hit; used direct invocation per brief's explicit allowance). Ran 3× consistently: **316 passed, 2 failed, 318 total** (2 failures = 1 distinct test doubled by duplicate file matching). Raw output in `🧪️w2-c-ts.txt`.

Sole failure: `workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm` — `Cannot find module '.../🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js'`; confirmed the `pkg/` dir doesn't exist on disk. Pre-existing, unrelated to C8/C9 — matches 1-C's report verbatim, not mine, left untouched.

## Files touched
`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`, `🧰️framework/🛍️products/💻️os/🟦️component.ts` (`AppChannelClient` region + its describe block only).
