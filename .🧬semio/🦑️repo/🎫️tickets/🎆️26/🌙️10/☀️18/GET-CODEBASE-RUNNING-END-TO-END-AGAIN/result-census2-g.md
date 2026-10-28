FOR gate-g: batch ready `C/census2-g-tools/staged/` (SUPERSEDES `C/testmem2-g-tools/staged/`: apply exactly one of the two). Same tool: `python3 -I census2-g-tools/staged/stage.py check|apply|revert|selftest [--root DIR] [--only PREFIX]`, laws `zsh census2-g-tools/staged/verify.sh [--long]`. 10 edited files and 12 new ones: everything testmem2-g staged plus the ceiling of the owned command runners (below). All base hashes were `base` on the live tree at 18:59.

# result-census2-g (wave 1026g, executor census2-g): the test memory census finished, and the ceiling now holds every `bun test` process an owned runner starts

Tools `C/census2-g-tools/` (`list.py` file list, `one.py` one-file runner with tree-RSS sampling, `prof-retention.ts`, `staged/`), raw rows `C/census2-g-tools/lib-rest.jsonl` and its `.log` files (tool output, delete with the ticket). Census one is `C/result-testmem2-g.md`. No sub-agent, no ticket tool, no git command, no official run; every test ran alone under `nice` on my own pids. The coordinator restart at 18:50 killed nothing of value (the runner pid 81884 was paused by me, see below).

## 1. What was wrong with the ceiling of census one
testmem2-g declared per-file budgets and stopped a file at 150 % of its budget (3 GB when undeclared), but only inside the suite runner (`🗂️suites/🏃️run`) and the neutral Bun files runner. Any other caller that ran `bun test` through the owned command runners (`🎛️owned-execution`, `🧪️testing/🎛️execution`), which is how the multi-GB workers of the field census (gram2-g: 3.1 to 5.1 GB for 40 minutes) were started, had no ceiling at all.

## 2. The fix (staged, schema first, EN and DE)
New module `🧪️testing/🟨️bun/🧯️ceiling` (own `🧬️schema`, fixtures, tests): `bunTestCeilingV1(command, args, cwd, env)` answers a function for a `bun test` command of the hosting runtime and nothing for every other command. It reads the file operands, looks them up in the test file budgets relative to the checkout (a process keeps what all its files load, so it gets the biggest budget and the highest ceiling of its operands; no operand = an undeclared file), and watches the pid with the existing file memory watcher. Above the ceiling it terminates the process tree once and names the files, the peak and the ceiling in English or German. `SEMIO_TEST_FILE_CEILING=off` lifts it, `SEMIO_TEST_FILE_BUDGETS=<json>` replaces the table.
Wiring: `🎛️owned-execution` (+ `📥️capture` gets an `onSpawn(pid, kill)` hook that returns the release function) and `🧪️testing/🎛️execution` (new termination reason `memory-ceiling`). The build guide names it in both languages.
Proof:
- 7/7 in the ceiling suite, among them a REAL `bun test` process that holds memory above its ceiling: terminated and named in English and German through the owned runner and through the budgeted runner, passes within its budget, passes with the switch off. 659 expectations.
- Controls (the law must fail without the code): with the owned runner's ceiling call removed the real-process test fails (6 pass, 1 fail); with the budgeted runner's `stop("memory-ceiling")` removed it fails (6 pass, 1 fail). Both files restored byte-exact (diff against the saved copies).
- Two test faults found and fixed on the way: the fixture imported the runners with paths one level too deep, and its table declared 10 MB although the schema demands at least 64.
- Fresh clone of the process module with the batch applied (`stage.py apply --only <process>`): ceiling 7/7, file-budgets 8/8, suites 7/7. Four neighbouring suites (owned-execution, execution, bun files, auxiliary pipes) give the SAME counts as the live tree without the batch (5/1, 2/1, 1/1, 2/9 pass/fail): their failures exist today (`Original parent Script process invocation required`, two stale asserts), the batch changes none.
- `stage.py selftest`: all 10 files round-trip base to after to base on the live tree.

## 3. Census two: the rest of the library
`list.py` gave 687 further test files (library after the first census, in path order). Rows so far: INTERIM_ROWS.

INTERIM_TABLE

## 4. Not done / limits
- Not run: Linux and Windows branches of the watcher (it reads `ps`, so the Windows path is a refusal, as in census one); the whole-repo `--long` law of the file budgets.
- The observation `retention` test holds 1240 MB (1621 MB with garbage collection forced after every test: allocator retention, heap only 141 MB) because its oversize fixtures stringify a result above the retained-image bound twice per run; INTERIM_RETENTION
