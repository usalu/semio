# Owner Cost Estimates Biased by Fleet Samples

## Root cause
`🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts`: `mergeOwnerCostTablesV1` pooled this machine's file with the fleet table (`🕸️graph/🤝️prefetch/🟦️.ts:90`), and `estimateOwnerCostV1` took the max of the newest samples regardless of who measured them. Samples from bigger hosts (lazy heap growth, shared warm-up, stopped lower bounds) beat the 647/1670 MB this 16 GB machine measured for repo-lib (fleet: about 8277 managed = 16554 MB footprint), so the owner was "heavy" and, above the solo grant, refused by a prediction.

## Changes
- Sample schema: `hostMegabytes` (physical memory of the measuring host) required (`💰️cost/🧬️schema/🔣️.json`, `OwnerSampleV1`); recorded by `recordOwnerCostV1` and the recorder (default `ownerCostHostMegabytesV1()`); part of the merge dedupe key. `ownerCostContextV1` carries `hostMegabytes`.
- `estimateOwnerCostV1`: judges by samples from hosts no bigger than the context host when any exist; all samples only otherwise (owner never measured here). No context host keeps the old behaviour (serving machine server).
- Old local cache files lack the field, read as empty and regenerate.
- (b) was already in place (`planSoloV1`: only a solo grant of 0 refuses; the governor's measured stop yields `graph.owner-too-heavy` in `📦️project-nodes/🟨️.mjs`); covered by the read test "above even the tight solo grant is attempted alone".

## Tests
`💰️cost` (new test, 20 pass), `measured-only` 3, `📖️read` 16, `two-platforms` 5, `🛠️server` 15 + priority 3, `too-heavy` 6, `fleet-read` 5: all pass.
