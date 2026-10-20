# 🚪️ Handoff: ordinary-command ingress takes an exclusive dependencies lease and walks the workspace three times

For: peer fleet of ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN (integrate). From: executor x-ingress, ticket 26/10/20/ARCHITECTURE-COURSE-ON-BAZEL. Full analysis: `result-x-ingress.md`. Tools and staged set: `x-ingress-tools/` (`T` = this ticket folder).

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `G` = `L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation`, `B` = `L/🏗️build/🚀️bootstrap/🟦️.ts`.

## Relation to your staged sets (nothing duplicated)

- `walkperf`/`walkperf2` (generation receipt) is the mechanism that makes a warm tree cheap. It is applied live. This handoff does not replace it, it repairs it: it never sealed a receipt (below) and its miss path is expensive and exclusive.
- `ingress-perf` (journal revision, roster, recheck window, fetched/scheduler receipts, custody session, lease stamp) is applied live since 15:12:32 and targets the CARGO ingress. It does not touch `ensureBuildEnvironmentV1`, the provider walk or the dependencies exclusive publish; this set is orthogonal and commutes with it (disjoint files; both only add files to the sealed closure).
- `show-perf`, `graph-perf`, `gperf`: `show projects` inference only. Not on this path.

## Defect 1 (LIVE FIX ALREADY APPLIED, one token): evidence writer exceeds the relative-create write ceiling

`G/💾️storage/🟦️.ts:144` wrote evidence in 262,144-byte chunks; `withArtifactRelativeDirectoryV1` allows 65,536 (`🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json` `RelativeCreateProtocol.maximumWriteBytes`). First write of the 20 MB frontier throws "Bounded closed relative write bytes required", a 0-byte `*.frontier.ndjson` is left behind, the error is logged as `[DATA] … receipt failed`. Result: no generation receipt has ever been sealed (seven 0-byte frontier files in `.cache/b/d/m/generation/`), so the fast tier never hits and every `repo` command walks the whole workspace. Proof: `x-ingress-tools/repro-write-limit.ts`. Live edit at 15:36: `262144` -> `65536` (recorded in `result-x-ingress.md` "Backend edits"). The staged set contains the same fix as an extracted, tested function.

## Defect 2 (staged): the miss path is exclusive and walks three times

On a receipt miss `B:80` returns `receipt-required` straight after the shared walk, `B:86-88` then requests the exclusive dependencies lease (queued up to 6 h, FIFO), and inside it a deferred/failed publish triggers a second and third complete walk under the exclusive hold. The exclusive request also blocks every later shared request, so one miss stalls the whole fleet (6 course-target processes: shared wait 3 min, second shared wait 5 min, exclusive wait 1.5 min, each holder 2-4 min; measured in `result-x-ingress.md`). A running dev server holds the dependencies image shared for its lifetime, so on a miss `repo run` would wait for it.

Staged fix (anchored, idempotent, `x-ingress-tools/apply.py`):
1. After a shared walk the command decides readiness itself: it confirms the walk by REPLAYING the walk's own recorded observations against the live tree (`replayJavascriptDependencyGenerationWalkV1`: capture with a sentinel implementation, verify in memory, require the walk's own `sha256`), and only if the replay cannot decide (deferred unsettled timestamps, mismatch) it does the old second complete walk (`confirmGenerationWalkV1`). Same soundness argument as walkperf (equal answers to every recorded query), cost seconds instead of a minute.
2. The generation receipt is then published BEST EFFORT under the SHARED image custody, serialised only against other receipt writers by a writer lease (`javascript-dependency-generation-writer:<ws>`, `timeoutMs:0`): busy means skip. No exclusive image lease, no queue, no wait (`publishJavascriptDependencyGenerationReceiptSharedV1` + `runGenerationWriterV1`). Same precedent as `🏗️build/📊️reporting` (shared store, writer lease, `store.recheck()` before and after). Retirement of superseded receipts/evidence honours a 10 minute grace so a reader that is verifying an old receipt does not lose its evidence.
3. The exclusive pass (parser receipt missing, i.e. after a dependency change) adopts the shared walk by replay instead of re-walking; its own receipt publication stays exclusive.
4. A partially written evidence file is unlinked instead of left behind (`writeJavascriptDependencyGenerationFilesV1`).
Exclusive requests on the ordinary path: 0 (was 1 on every miss). Complete walks on a miss: 1 (was 3). Warm hit: verify only, no walk, no exclusive.

## Apply order (all anchors verified against the live tree 15:5x: `python3 x-ingress-tools/apply.py check` lists 24 pending operations, every anchor occurs once)

1. Apply every other staged change that edits files of the sealed closure first (any closure edit costs one walk and one deferral).
2. `python3 T/x-ingress-tools/apply.py apply --dry-run`, then `python3 T/x-ingress-tools/apply.py apply`. Edits: `G/🟦️.ts` (append 4 pure functions), `G/💾️storage/🟦️.ts` (15 anchored edits incl. the files writer, replay, shared publish, grace), `B` (5 anchored edits), 4 new files under `G/🧪️tests/🚪️ingress/`. No schema of the generation protocol changes.
3. `sh T/x-ingress-tools/check.sh <scratch dir outside the repo>` (mirror, explicit-file `bun test` of the new suite 10 tests, tsc over the 4 touched files).
4. Explicit existing suites (sequential): `B/../📦️parsers/🧪️tests/🟦️.ts` (the AST law: the `if(!publishReceipt)` branch and the tail are byte-identical, and the number of catch-with-return clauses stays 2) and `G/🧪️tests/🟦️.ts`. In the mirror both have the identical failing sets before and after (2 parser tests need the real envelope, 1 generation law needs the real tree).
5. Validate live: first `bun repo --tool=bazel run @teaching/architecture-course:test --dry-run` (one walk, `generation/` gets `*.receipt.json`), second run: only `dependencies:generation-verify:*` progress and no `native-fixed-image:.cache/b/d (exclusive)` line; `x-ingress-tools/timed.ts` prints the per-phase table.

## Not covered / open

- The receipt seals the 176-file runtime closure (130 process modules). Any edit to one of them, by any fleet, makes the next command walk once, and a process that started before the edit cannot publish (`implementation-unsettled`). `x-ingress-tools/closure-churn.py` shows 94 of 176 files changed within 24 h. This is sound and intended; it only matters while the integrate series edits those modules. After the freeze the receipt stays valid.
- Cross-process single flight of the walk (N parallel `repo run` all walking on a miss) is designed in `result-x-ingress.md` but not staged: it needs a walk lease held across walk, replay and publish.
- Taxonomy admission of the new test member `🧪️tests/🚪️ingress` (`🧬️schema`, `🐍️oracle.py`) is not checked (no `bun repo` taxonomy run); the file names follow the existing `🧪️tests` pattern.
- Not run live: the patched bootstrap against the real Store (frozen). Replay and shared publish are covered with real custody objects, the real lease library (foreign SQLite transaction as the third party), the real relative-create writer with evidence up to 5 MB, and source laws for the bootstrap edits.
