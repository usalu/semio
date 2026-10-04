# Native Compiler Layout Provenance

The original receipt collector recognized `package-hash` build directories. The current Cargo layout also uses `package/hash/out` beneath both `debug/build` and `intermediate/debug/build`. The original collector therefore omitted some actual dep-info units. Its retained counts are observations of that limited matcher, rather than a complete local compiler closure. Original receipts and actual command logs remain unchanged.

The binding collector now admits both layouts and retains complete dep-info text, checksum inputs, original captured preimages, later current bodies, and full mismatching before/current pairs with exact forward/inverse patches. Compiler emission selection remains bounded by each actual immediate fence and terminal time. A later receipt has its actual later observation time; it is never relabeled as an original post capture. A variant overwritten by a subsequent compiler cannot be reconstructed as originally retained.

| Actual run | Runtime scope | Original limited observation | Corrected observation |
| --- | --- | --- | --- |
| IO47160 | Whole library compiler RED: missing canonical ArtifactRef trait | 2 units/11 inputs | Historical incomplete matcher retained; no inferred complete closure |
| UI60430 | Whole library compiler RED: missing canonical viewport traits | 1 unit/2 inputs | Historical incomplete matcher retained; no inferred complete closure |
| IO91527 | Whole library GREEN8/8 | 1 unit/9 inputs | Later observation: 2 emitted units/29 inputs; original pre0, current1, owned0 |
| UI81429 | Whole library6/7; new f64 comparison assertion RED | 0 units/0 inputs | Separate pre-correction receipt55059:1 emitted unit/11 inputs; pre/current/owned0 |
| UI2442 | Same whole library GREEN7/7 after owned assertion correction | Corrected matcher used | Original successor post39135:1 emitted unit/11 inputs; pre/current/owned0 |
| OS91951 | Original registered artifact-kind owner law GREEN1/1 | 1 unit/619 inputs | Historical limited observation; runtime selection remains1 with1255 filtered by the original route |

The later IO mismatch is `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️.graphql`. Its full original captured/current text and hashes are retained with unknown writer attribution. This is a current source mismatch, not an IO runtime failure or a claim that current foreign bytes are equivalent. The binding's27 edited owner rows remain exact under the explicit f64 successor and scoped GUI authority.

Earlier General and core receipts used the same limited matcher. Their actual runtime outcomes remain independently supported by complete command logs and rosters. Their old compiler counts must receive the same qualification; no whole current source stability follows from those counts.

Corrected binding collector: `binding-native-inputs/📜️script.ts`. Durable receipts and logs: `🗑️generated/current-native-worker`. Final per-owner evidence: `🗑️generated/neutral-field-binding/actual-execution-1/binding-final-results-1.json`.
