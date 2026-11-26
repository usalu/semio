# F96: macos-reading, walk-edits, heavy-questions green

Lane F96, 2026-11-26. All three files pass on two consecutive `repo test-file` runs (macos-reading 4/0 twice, walk-edits 9/0 twice, heavy-questions 17/0 twice in a row at the end). `repo typecheck` of the touched paths: passed, 0 errors; `dangling-imports.py`: exit 0. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `G` = `B/🕸️graph`.

## macos-reading (`G/🧪️tests/🙋️fleet-read/🍏️macos-reading`) 1/2 -> 4/0

Root causes (two, both live numbers):
1. `fleetReadStaleGenerationV1` judged owner costs by `ownerCostContextV1()`, whose ceiling comes from the real `totalmem()` of the machine running the test, although the test declares the host (`hostMegabytes`). On a bigger machine the seed of `@semio-tech/s-dev-composition` (13976) was no longer saturated and showed up as a second heavy project ("27952 MB").
2. The test hard-coded numbers of an older policy: the parent floor was lowered from 3800 to 3072 MB (commit 3cde99d423d, 2026-11-18), so the grant is 4096 MB, not 3512, and the solo grant 9010, not 8428.

Fix: `G/🤝️prefetch/🟦️.ts`: when the caller declares the host, the cost context (ceiling) is built from that declared host (`ownerCostCeilingV1(reading.hostMegabytes)`); the production path without `hostMegabytes` is unchanged. The test derives every number from the same policy tables the plan reads (`ownerGrantV1`, `fleetGrantMegabytesV1`, `inferableProjectsV1`, `soloGrantV1`, `describeHeavyV1`) and asserts the full plan and solo line through the bilingual templates (`ownerCostTextV1`), in English and German. The 4 GB case no longer has a heavy project (every seed is saturated by the 1 GB ceiling), so it asserts the light `wantedTextV1("plan")` line.

Not fixed (same class, outside the assignment): `G/🧪️tests/🙋️fleet-read/🌍️small-booking` has 1 failure with hard-coded "6200 MB ... 192 ... " numbers on the live machine memory (reads `fleetHostReadingV1` of the real host; its grant is 3880 MB here, 2 heavy projects). It needs the same derivation, with an injected host reading.

## walk-edits (`G/📥️inference/🧪️tests/🧭️walk-edits`) 6/3 -> 9/0

History (read-only): the observation of a command walk is one `closure-graph` global row (`{digest, answer}`) judged by `closureGraphFactHoldsV1`. Its documented contract (`script-inputs/🟨️.mjs`): a question that cannot be decided without visiting modules does not hold in the synchronous judge (module nodes carry no witness), and the asynchronous judge prepares it first. Production did not regress; the test was stale in two ways.
1. The check phase used the synchronous `observationRowsHoldV1`, which answers false for every edit that needs a visit (a comment, a new file next to the closure). The verifier's real judge is `prepareGlobalFactsV1` + `observationRowsHoldAsyncV1` with the walk's cache bound (`bindClosureScriptsV1`). The test now runs that path.
2. Fixture step "an import of a file the walk already reaches" expected rows to be stale (`rowsHold:false`) with an unchanged roster. The row holds while the answer through its view is the recorded one after the digest moved (documented), and the title of the law is "hold exactly when a fresh walk answers the same", so the fixture is now `rowsHold:true`. Every step now has `rowsHold == !rosterMoved`.

## heavy-questions (`G/📥️inference/🧭️script-inputs/🪆️heavy-questions`) 15/1 -> 17/0

Root causes:
1. Wall-clock laws (silence stop `<10000 ms`, dead child `2900..15000 ms`, busy child `>3000 ms`, prime lane `<2500 ms`, orphan `<6000 ms`) jittered under fleet load.
2. Live numbers: `preflight:39/39` and `checked > 30` of the live tool-closure count (now 40).
3. Production: the preflight child peaks at 3.06 GB RSS on the current tree (measured with `/usr/bin/time -l`), above the 2560 MB cap, so `the preflight of the capped child accepts the real tree` and `the capped child turns its event loop` failed whenever GC did not run in time (3 of 9 runs, exit 137 "resident memory cap of 2560 MB exceeded").
4. Production: stdin of a child killed before it read its request raised an unhandled `EPIPE`.

Fix, production (`🟨️.mjs`, `🟨️.d.mts`, `🔣️.json`):
- `ports` of the watch: `now`, `after`, `every` (return their canceller) and `start` (the spawner), defaulting to `Date.now`, `setTimeout`, `setInterval` and `child_process.spawn`; the watch logic is unchanged.
- `child.stdin.once("error",()=>{})`: the close event already reports a dead child.
- `memoryLimitMegabytes` 2560 -> 4096 (the schema only requires >= 256; the only reader is this module). Decision for the coordinator: this raises the cap of every heavy-question child.

Fix, test: a manual time harness (`manual()`) drives the watch and logs every kill with its manual time. Laws now assert exact moments instead of durations: silent child killed at exactly the silence window (nothing at window-1), dead child at exactly silence x scale (3000 ms for scale 2), living, leased and cpu-progressing children never killed while the manual clock runs past 2 to 3 windows (children are released by a `go` file, a new line of life is awaited before each advance, so no race with bursts). The lane law asserts ordering (the question ends while the first prime is unsettled; the second prime starts after the first ended) with an inert clock. The orphan law waits for death without a time bound (the one-second guarantee lives in the watchdog's `watchMilliseconds`, whose own test is `🐕️watchdog/🧪️tests/🪦️busy-orphan`). The preflight laws assert structure (same total in every beat, steps only advance and progress) and `checked > 0`; the child cap in the two direct spawns comes from the policy, not "2560".

## Files (sha256 prefix)

See the landing entry in `T/landings.md`.


# Round 2

## Cap policy-driven (heavy-questions)

`memoryLimitMegabytes` (constant) is gone. Schema-first in `🧬️schema/🔣️.json` and `🔣️.json`: `memoryFloorMegabytes` 2560, `memoryCeilingMegabytes` 6144, `concurrentChildren` 2. `heavyQuestionCapV1(policy, reading)` (exported, in `.d.mts`) = the solo grant of the graph-read machinery (`soloGrantV1` over `fleetHostReadingV1`: host memory less parent and system reserve, available memory less the available floor, so free memory and reserve both bind) divided by `concurrentChildren`, clamped to floor and ceiling. The cap is handed to the child through the environment variable. A child that outgrows it exits 137 with `resident memory cap of N MB exceeded (resident R MB)`; the parent raises the bilingual refusal `graph.heavy-question-memory` (texts catalog) naming limit, resident, host, available, children, floor, ceiling, question and phase. New `host` port for laws. Tests: cap law (floor for a starved host, ceiling for a huge one, monotone in free memory, children together leave the reserve), env law, refusal law in en and de.

## Why the preflight child peaked at 3.06 GB (heap census)

Measured with `/usr/bin/time -l`, `bun:jsc` heap stats and a V8 heap snapshot taken after the second closure: heap about 660 MB plus 430 MB external, resident 2.1 to 2.5 GB, flat from the first closure on (the steady state, not growth). 2.38 million strings (448 MB) of which 2.3 million are absolute paths, but only 72,055 distinct: 1.26 million were duplicates. Retainer walk (three levels): the paths sit in arrays of `witness.typed` and `witness.paths` of the module-graph records. Cause: `observationWitnessV1` rebases every observation row, which makes a new copy of the same few thousand paths per node (7,400 nodes). Not the TypeScript program store, the shared source-file store, the script input cache maps, the closure graph or the row table (each cleared and measured, none moved the heap).

Fix (retention, not cap): `👁️observation/🟨️.mjs` shares one copy of each witness path and typed fact (`internWitnessStringV1`, bounded table of 262144 entries). Peak resident of the preflight child: 3.06 GB -> 2.17 GB (same tree, same answer `checked 40, refused []`). Tried and reverted: one snapshot scope per closure released memory but the walk lost its memo sharing (more than 170 s instead of 28 s).

## small-booking

`fleet-read/🌍️small-booking`: host injected (`hostMegabytes`), every number and line derived from the policy tables and the bilingual templates, as in macos-reading. 3/0.

## Test counts (`repo test-file`, two runs each, load average 50 to 65 on the machine)

| File | Run 1 | Run 2 |
|---|---|---|
| macos-reading | 4/0 | 4/0 |
| small-booking | 3/0 | 3/0 |
| fleet-read (suite file) | 5/0 | 5/0 |
| walk-edits | 9/0 | 9/0 |
| observation witness | 5/0 | 5/0 |
| heavy-questions | 19/0 (386 s) | 19/0 (745 s) |

Not mine, red in both runs: `📖️texts/🧪️tests` 2/1, `closure.derived-provider-differs is unused` (a key of another lane). `repo typecheck` of the touched paths: passed, 211 files, 0 errors. `dangling-imports.py`: exit 0.

## Process notes

Lock `observation` released as soon as the witness edit was done (14:14); on release I also removed `landing.lock.graph` by mistake (it had been taken over by F99) by the time I tried to re-create it another lane already held `graph` again (my mkdir found it present, nothing of theirs was touched). F99 should check that its lock folder is the one it expects.
