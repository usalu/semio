# 🔬️ Boot Matrix 1904 Audit

Read-only audit of the static Playwright run (`🗑️generated/static-playwright.json`, run start 2026-11-23T02:51:33Z, 770 s). Parser: `analyze-boot-matrix-1904.cjs` in this ticket folder. Test source: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`.

Totals: 16 tests, 4 passed (energie en/de, statik en/de), 12 failed.

## Verdict on the four panes

- **aggregator (en, de): fails only on (a).** The test body passed. The only failing assertion is the soft `unexpected browser console errors` check, and its contents are only `[DEBUG] leaf prior_slots` and `[DEBUG] marker` lines (7483+204 de, 7459+954 en). Line counts reconcile exactly with Playwright's diff size. No poll_wit or other lines.
- **aussuchen (en, de): NOT explained by (a) or (b).** The pane shell never sets any readiness beacon, and the test dies at `waitForPaneShellOutcome` (test line 630, `waitForFunction` timeout of 120 s, `SHELL_READY_TIMEOUT_MS`). The error-context shows the pane rendered with a banner: "semio · sourcing · curation has not answered "setContributions" for 118 s". No console line explains the stall.
- **bearbeiten (en, de): NOT explained by (a) or (b).** Same failure as aussuchen: no readiness beacon within 120 s. Snapshot shows "semio · process · 3d has not answered "setContributions" for 118 s".
- **verfolgen (en, de): NOT explained by (a) or (b) alone.** The failing assertion is the pane shell readiness check (test line 631), which returned `"error"` after about 4.5 s. The snapshot shows the toast "Die Eingabe konnte nicht zugestellt werden." / "The input could not be delivered.". The only non-noise console line is the poll_wit refusal (b), and it does not by itself explain the shell error. The artifacts do not show a `setContributions` failure for verfolgen, so its root cause is not established.

## Per-test table

Noise counts are from `[DEBUG] leaf prior_slots` / `[DEBUG] marker`. Non-noise console lines are listed separately.

| Pane | Locale | Status | Duration | Failing assertion(s) | Noise leaf / marker | Other console (count) |
|---|---|---|---|---|---|---|
| generator | de | failed | 3300 ms | shell `"error"` at line 631 (expected `"ready"`); soft console check | 0 / 0 | poll_wit refusal x1; `setContributions command failed procedural original-turn.native-return-refused: authority` x1 |
| generator | en | failed | 4111 ms | same | 0 / 0 | same |
| koordinator | de | failed | 10036 ms | shell `"error"` at line 631; soft console check | 25127 / 22472 | `setContributions` `interactive-job.publication-stalled` (advanced 4096 units without moving ladder) x1; poll_wit refusal x1 |
| koordinator | en | failed | 11400 ms | same | 25127 / 22472 | same |
| aggregator | de | failed | 3360 ms | soft console check only | 7483 / 204 | none |
| aggregator | en | failed | 4660 ms | soft console check only | 7459 / 954 | none |
| energie | de | passed | 7404 ms | none | none | none |
| energie | en | passed | 10278 ms | none | none | none |
| aussuchen | de | failed | 120325 ms | `waitForFunction` timeout 120 s at line 630; soft console check | 7972 / 9677 | none |
| aussuchen | en | failed | 120389 ms | same | 7826 / 9677 | none |
| bearbeiten | de | failed | 120331 ms | `waitForFunction` timeout 120 s at line 630; soft console check | 7950 / 6485 | none |
| bearbeiten | en | failed | 120305 ms | same | 7937 / 6485 | none |
| verfolgen | de | failed | 4634 ms | shell `"error"` at line 631; soft console check | 7082 / 8953 | poll_wit refusal x1 |
| verfolgen | en | failed | 4506 ms | same | 7082 / 8953 | poll_wit refusal x1 |
| statik | de | passed | 105838 ms | none | none | none |
| statik | en | passed | 105736 ms | none | none | none |

Koordinator's noise count is within one line of Playwright's diff size (47603 received); the difference is a formatting line, not a missing message.

## Page errors and local-dependency requests

- No test reported `unexpected browser page errors` or `release boot requested local development services`. Both are soft assertions, so their absence means zero page errors and zero local-dependency hits. Caveat: the local-dependency observer only runs when `DEMONSTRATOR_VERIFY_CDN=1`; the JSON does not record that env value.

## Shell and pane readiness state (from error-context)

- **generator (de/en):** shell `"error"`. Snapshot shows region "Generator" with the error toast and the dock/navigation rendered. Only `setContributions` failure is the native-return-refused line.
- **koordinator (de/en):** shell `"error"`. Snapshot shows the error toast. `setContributions` publication stalled error.
- **verfolgen (de/en):** shell `"error"`. Snapshot shows the error toast, four status regions, and the pane.
- **aggregator (de/en):** shell reached `"ready"` (test passed the beacon). Region "Aggregator" / "Aggregator" rendered.
- **aussuchen (de/en):** beacon never set within 120 s. Pane rendered (Editor, "Aussuchen" / "Select"), with the 118 s `setContributions` stall banner and a "Befehl abbrechen" / "Cancel command" button.
- **bearbeiten (de/en):** beacon never set within 120 s. Pane rendered, with the same 118 s `setContributions` stall banner.
- **energie, statik (de/en):** passed, no error-context.

## Root-cause groups

1. **Noise only (not a failure):** aggregator de/en. Caused by the two `[DEBUG]` prints. Not a real regression.
2. **Shell boot error, `setContributions` family:** generator and koordinator (de/en). Both show a `setContributions` error on the console (native-return-refused and publication-stalled) and the "input could not be delivered" toast at readiness time. Likely one shared cause (the boot-time `setContributions` path), not confirmed in source.
3. **Shell never ready, `setContributions` stalled:** aussuchen and bearbeiten (de/en). Boot hangs on a `setContributions` command that the banner reports as unanswered for 118 s. No console error is logged. The beacon wait times out after 120 s. Not confirmed whether the stall is the cause of the missing beacon or a later effect.
4. **Shell boot error, cause not visible:** verfolgen (de/en). Same error-toast pattern as group 2, but no `setContributions` error is logged. The only non-noise console line is the poll_wit refusal (b). Because the shell error is a failure that is neither (a) nor (b), verfolgen must be treated as a real failure until the shell error is traced to a cause. The poll_wit refusal is a candidate but unproven.

Note: group 2 and group 4 share the same visible symptom; group 2 has a clear `setContributions` cause in the log, group 4 does not.

## Scripts and files

- Parser: `analyze-boot-matrix-1904.cjs` (ticket folder root). Reads only the JSON and does not modify any source.
- Source reviewed (read-only): `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` lines 40–175 and 610–660.
- No source files were edited.
