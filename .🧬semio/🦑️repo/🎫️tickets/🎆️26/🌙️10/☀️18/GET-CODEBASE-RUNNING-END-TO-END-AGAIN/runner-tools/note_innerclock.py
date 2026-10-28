"""📝 Replaces the GO RED finding line of result-runner.md with the innerclock ready line (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
s = t.index("FOR coordinator: GO RED = cbudget gap")
e = t.index("\n", s)
line = ("FOR coordinator: innerclock READY (5 files; GO RED = cbudget gap): `repo-cli-go:test` panicked \"test timed out after 15s\" in `TestGoalsMcpResource` at load ~20 because "
        "`📚️library/🧪️testing/🎚️levels/🟦️.ts` `goLevelTestArgs` hands `go test -timeout 15s` (whole-binary WALL clock) and Go tests run under `runCanonicalGoTests` (compiler admission, not runBudgetedTestCommand), "
        "so G46's contention clock never reaches them; same for pytest `--timeout`, dotnet blame-hang, vitest/bun per-test, Playwright per-spec. Fix: every inner timeout = level budget × `budgetStretchV1(ratio)` with "
        "`ratio` = `sharedHostContentionV1()().ratio` at launch (new trailing parameter, callers unchanged; idle host = the old vectors; ceiling 8× = the cbudget policy); `budgetStretchV1`/`sharedHostContentionV1` "
        "re-exported through `🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts`; the corpus law now pins ratio 1 (it called the defaults). New law `🎚️levels/🧪️tests/⏲️stretch/` (Python oracle over "
        "4 levels × 7 ratios + go 45 s/120 s/15 s): base 2 red → 10/0 together with the corpus law; tsc 0 incl. a Go caller; scan 4/0. Apply: `python3 -I C/runner-tools/staged/innerclock/stage.py apply` ; "
        "verify: `sh C/runner-tools/staged/innerclock/verify.sh /Users/ueli/Documents/semio` ; revert: `… stage.py revert`. Product proof: `python3 -I C/prodmap-tools/group.py pm-go "
        "'@semio-tech/repo-cli-go,@semio-tech/repo-workspace-go,@semio-tech/repo-graphql-go,@semio-tech/repo-coordinator-go,@semio-tech/repo-tickets-go,@semio-tech/repo-launcher-go,@semio-tech/repo-statutes-go' 'test'`.")
open(p, "w", encoding="utf8").write(t[:s] + line + t[e:])
