# Audit A5 (wave 1125): re-run of landings after the 17:27-17:31 test-file outage

Run one file at a time, load1 gated < 45. Logs are in the scratchpad (not in the ticket).

| Landing | File | Result |
|---|---|---|
| F6 | generations/tests/journal/🟦️.ts | PASS 9/0 |
| F6 | generations/tests/journal-memory/🟦️.ts | PASS 1/0 |
| F6 | budget/tests/🟦️.ts | FAIL 21/2: "the nx partition provisions its own backing and charges the artifact envelope" ("Actual Native platform image required"). Same known unrelated failure as in the landing. |
| F6 | budget/opening/reset/tests/🟦️.ts | PASS 13/0 |
| F6 | doctor/tests/probes/🟦️.ts | PASS 23/0 |
| F16 | remote-cache/tests/🟦️.ts | PASS 27/0 |
| F16 | remote-cache/fleet/tests/🟦️.ts | PASS 14/0 |
| F16 | shared-worktree/topology/tests/🟦️.ts | PASS 74/0 |
| F16 | unit-cache/remap/tests/🟦️.ts | PASS 11/0 |
| F16 | unit-cache/store/tests/generated/🟦️.ts | PASS 2/0 |
| F20 | authorizers/tests/ancestry/🟦️.ts | PASS 9/0 |
| F20 | observation/tests/carried-verdicts/🟦️.ts | PASS 1/0 |
| F20 | observation/tests/🟦️.ts (roster) | FAIL 62/7. Failing: "conflicting repeated actual reads are retained and inference is refused", "captures original facts inherited from cached project inference", "conflicting cached project facts cannot overwrite a parent's consumed read", "semantic source analysis" (3 tests), "semantic parent validation shares each provider independently across project images". First error: `expect(received).toThrow(expected)`, expected "Conflicting inference observations". Landing said 68/1, so this is a regression since then (not investigated). |
| F16 | server 421 instance test | No test in tree. remote-cache/🖥️server/🟦️.ts has the 421 guard (`foreign`, line ~154). The test exists only staged at T/🧰️scripts/F16/instances-test.ts. |
| F17 | repo run @semio-tech/repo-launcher-go:test | RAN, FAILED (Nx task go:test exit 1). Package github.com/usalu/semio/repo/launcher: FAIL, `TestLauncherReadsItsStateBelowTheDeclaredDirectory` (zz_semio test file line 87, "tools partition ..." message, the test reads a partition path under .cache/b/o/m/tests). No `ok` line for the workspace package in the log, so it is unconfirmed whether it ran. Full log: scratchpad a5/f17-go.log. Run took ~20 min in background. |
| F22 | (none) | No "Wave 1125 F22" section in landings.md. coordination.md 17:30 says F22 had a stray `}` in budget/opening/🟦️.ts; not re-checked here. |

Not run: F22 landing (no section), the staged 421 test (not in tree).

