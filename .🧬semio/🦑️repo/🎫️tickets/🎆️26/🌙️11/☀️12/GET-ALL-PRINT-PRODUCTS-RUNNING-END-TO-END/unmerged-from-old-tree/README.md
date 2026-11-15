# Unmerged Edits From The Old Worktree (2026-11-13)

Seven files were changed both by this ticket's session in `/Users/Shared/semio` (18:34–18:41, graph-plan starvation fix, see `graph-plan-starvation.md`) and by the fleet (18:09–18:19). The canonical tree keeps the fleet version. The old-tree version of each file is stored beside this note, byte for byte, under the number below. The content of the old-tree versions is in no blob of the store, so these copies are the only ones once the old worktree is gone.

The rest of that fix was copied into the canonical tree (tests `🏡️resident/🧪️tests/🛟️reservation/`, `🌡️host-capacity/🧪️tests/🛟️unsatisfiable/`, `🏡️resident/🧬️schema/🔣️.json`, `🏡️resident/🧪️tests/📨️protocol/🟦️.ts`, `🕸️graph/🕰️generations/🟨️.mjs` and `.d.mts`), so those tests expect the implementation that is only in the files below: re-apply the fix on top of the fleet version, then delete this folder.

| File here | Path in the worktree | Old tree only | Fleet only |
|---|---|---|---|
| `1.ts.txt` | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts` | 4 lines | 8 lines |
| `2.ts.txt` | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` | 18 lines | 16 lines |
| `3.json.txt` | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json` | 16 lines | 53 lines |
| `4.ts.txt` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts` | 5 lines | 11 lines |
| `5.mjs.txt` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` | 14 lines | 13 lines |
| `6.mts.txt` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.d.mts` | 1 lines | 1 lines |
| `7.mjs.txt` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs` | 7 lines | 7 lines |

## Merged (2026-11-13, canonical tree)

All seven were merged by hand on top of the fleet version (nothing overwritten); the stored copies stay as the reference.

- 1 gate: the wait loop keeps the fleet's `never`/`starved` refusal and `shrunk` line and adds the lone-request fallback (`unsatisfiableHostRequestV1`, `lone`, `unsatisfiable` line). Import extended.
- 2 decision: added `unsatisfiableHostRequestV1` and the `unsatisfiable` text name; the fleet's `minimumMegabytes`, `shrunk`/`never`/`starved`, `light` rule stay untouched.
- 3 host-capacity schema: added `unsatisfiable` to the text name list, the text property and the en/de texts; the fleet's `minimumMegabytes`, `shrunk`, `never`, `starved`, `memoryNeverFits`, `memoryStarved` stay.
- 4 library host-capacity: nothing of ours was left to apply: the fleet's `reserveGraphPlanCapacityV1` already returns the granted `megabytes` (plus `minimumMegabytes`); only its docstring mentions the bypass fallback now.
- 5 slice-pool: fleet booking (`sliceBookingV1`, `minimumMegabytes`, `reserving` progress, worker memory budget from a smaller grant) kept; ours added on top: `sliceAdmissionV1` clamp (only when the booking has no minimum, because the gate shrinks requests with a minimum itself, and only when `hostShareBasisPoints` is configured), `reduced` progress decision, wave share from the granted/booked ratio passed to the worker as `SEMIO_SLICE_WAVE_BASIS_POINTS`.
- 6 limit `.d.mts`: `sliceAdmissionV1` declaration added beside `sliceBookingV1`.
- 7 limit `.mjs`: `sliceAdmissionV1` added beside `sliceBookingV1`.
- Resident schema (`🏡️resident/🧬️schema/🔣️.json`, copied earlier from our tree, so it had lost the fleet side): re-added `sliceHostElasticity.hostCapBasisPoints` 4000 and `minimumMegabytes` 2048 and the en/de text `host-slice-owners-reserving`, reconstructed from the fleet tests `📐️booking` and `🛑️cancel`; removed our `minimumReservationMegabytes` (superseded by `minimumMegabytes`).

Tests: reservation 8, unsatisfiable 7, booking 3, shrink 6, resident protocol 23, pressure 7, cancel 37, waves 5, host-capacity library 17, graph-plan 4, gate 31, upgrade 10, followup 11, phases 10, estimates 14, quiet 4, unmanaged 4, blockers 5, handover 9, liveness 2, process host-capacity 24: all pass. Fail (not ours, the fleet's request-kind change in the library host-capacity module): `🏊️farm` 3, `👥️agents` 1, `🧾️label` 1.
