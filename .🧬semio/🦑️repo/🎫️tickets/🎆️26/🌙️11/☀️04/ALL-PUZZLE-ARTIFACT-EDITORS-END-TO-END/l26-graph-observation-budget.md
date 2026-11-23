# L26 graph observation budget

No coordinator PROTOCOL.md found (mmin -2880).

## Root cause
`observationBlockPolicyV1.maximumAggregateNodes` was 4194304, equal to `maximumAggregateRows`, but one observation row costs about 11 JSON nodes (object, 6 strings, commas) as counted by `observationJsonV1`. A record is charged header + result + every block against one budget (`readInferencePhysicalRecordV1`), so a record of roughly 380k+ rows (tree has ~226k source observations, 1206 owners, plus result) exceeds it, and `observationChargeV1` throws. That error is not a damage error, so `readInferenceRecordOrMissV1` rethrows and the whole request fails.

## Fix
Policy in `observation/🧬️schema/🔣️.json`: `maximumAggregateNodes` 4194304 -> 268435456 (= `maximumAggregateOutputBytes`). A node needs at least one byte, so the byte budget already bounds nodes; the node budget can never be tighter than it. Rows cap (4194304) x 16 nodes also fits. No code change; the miss path (b) was left strict on purpose: an over-budget record is a policy error, not damage.

## Tests
New `observation/🧪️tests/🧮️aggregate-nodes/🟦️.ts` (3 tests, passed via repo test_file): budget admits max rows x16 nodes, is >= aggregate output bytes, admits 226000 rows x 22 nodes.

Not done: the running graph service was not restarted, so it must reload the schema to pick this up. Per-JSON `maximumNodes` (4194304) unchanged; a single result JSON above that would still refuse.
