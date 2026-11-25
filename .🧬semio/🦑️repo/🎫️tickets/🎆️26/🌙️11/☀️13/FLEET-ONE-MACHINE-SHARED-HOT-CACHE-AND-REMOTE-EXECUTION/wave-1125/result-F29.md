# Result F29

## 1. 421 instance guard test
Landed `🌐️remote-cache/🖥️server/🧪️tests/🧬️instances/🟦️.ts` (from `T/🧰️scripts/F16/instances-test.ts`; title no longer claims deletes, only PUT and slots are guarded). `repo test-file`: instances 2/0, server tests 10/0, access 8/0. Guard unchanged: no defect. Lock `remote-cache` taken and released; dangling-imports exit 0.

## 2. Go suite
- Not a hang: `repo run @semio-tech/repo-launcher-go:test` spends 5-13 min before nx starts (graph slice read, `source.served`, repeated passes whenever a path changes under the read; load 35-60), then nx itself takes 30-55 s. Cause is the graph read under landing churn, not Go; no cheap tooling fix (quiet window helps). The earlier "exit 0 without tests" was the broken graph service (BUILD_SOURCE_UNPARSEABLE).
- A5's failure `TestLauncherReadsItsStateBelowTheDeclaredDirectory` was NOT from F17: `t.TempDir()` under the sandbox exceeds `machineStateMaximumCharacters` (120), so the launcher rightly ignored the declared state dir. Test now takes the state dir from `os.MkdirTemp("", "ms-")` (like its sibling). Product code unchanged.
- Results: `repo-launcher-go:test` exit 0 (`ok .../repo/launcher`), `-v -run Hidden`: `TestCapturedHelperIsHidden` PASS (F17 hidden-children). `repo-workspace-go:test -- -v`: exit 0, 176 PASS / 0 FAIL, including `TestToolCommandIsHidden` (F17 hidden-console). Both are Unix branches only; Windows branches unverified (needs g1-m05).
- Note: `repo run <target> -- -v -run X` passes Go args through.
