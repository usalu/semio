# Package P1: restarts that throw work away (2026-11-14, built in scratch, NOT LANDED)

T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. No graph or build command was run (operating rule); every claim below is from code or from a bun test file.

## Verdicts

1. The restart seen on g1-m01 ("Dependency publication requires fresh process implementation admission", then "Restarting with the current implementation (1 of 6)") is NOT about publishing dependencies. It is the Original's check of the private Nx projection sources (`admitProjection` in `LIB/⚡️caching/🚀️bootstrap/📜️script.ts`, ~15 files + the owner-argument sources): it compares the files as the Original loaded them (at its start, before the 2 344 s graph plan) with the disk, and any byte change anywhere in them throws the readmission request, exit 76, which the sealed execution layer renders with the misleading "dependency publication" text and the door turns into a restart of the whole command. A worker plans for tens of minutes while the replica brings peer edits, so this fires almost surely. FOUND, FIXED IN SCRATCH.
2. Fixed: a CODE-only move of those files is now adopted by a fresh process (no restart, no second plan); a moved CONTRACT file (`*.json`), a move after the Nx process was handed the identity, or a fresh process that fails still restarts, with a bilingual message that names the files. Proven by 4 new/changed bun test files (below). NOT LANDED: `N/LAND-OK` does not exist.
3. "Can admission be granted to a fresh CHILD instead of restarting the parent": YES, and that is what P1 builds. The Nx process was always a fresh process loading the current files; only the identity token the Original attested was stale. A fresh process evaluates the current sources (about 60 ms warm, measured) and the Original adopts the identity and revisions it reports.
4. Completed owners survive an implementation change in the middle of a read: PROVEN at generation level (new test in `🌊️waves`, three digests: code-body edit, data-row change, unchanged). The resume computes exactly `COUNT - ownerWave` owners, none of the finished wave twice, and the final graph equals the independent Python oracle.
5. Why the second plan on g1-m01 began at "owners 0 of 703": NOT PROVEN (no service log of that worker was collected). Code-derived candidates and how to decide are below. The restart itself is not the reason the 2 344 s are lost; the reason is what the second plan found stale.
6. Restart only if the remaining work can change: for the Nx projection implemented (code-only vs contract). The other triggers are listed with the rule each uses; narrowing them is NOT done (findings below).

## Restart triggers (verified in code)

Outermost relaunch: `runBuildWithRestartV1` (`LIB/🏗️build/🚪️door/🔄️restart`) -> `runWithStaleReexecV1` (`🧷️format-stamps/🔄️reexec`): a stale failure of the outermost command relaunches the same command line (max 6; a run longer than 600 s resets the count; the graph-queue handover is not counted). Everything the command held in memory, incl. its plan, is gone; only what was published under a key survives.

| # | Trigger | Compares | Why (correctness) | Throws away |
|---|---|---|---|---|
| 1 | Nx projection admission, `admitProjection` (script.ts) + `withDependencyReadmissionV1` (exit 76 -> stale) | sha of the ~15 projection sources loaded vs disk, before every Nx launch | the Nx process loads the files fresh and checks its own identity against the one the Original attested (`Original private Nx dependency implementation differs`) | whole command incl. the 2 344 s plan (FIXED for code-only moves) |
| 2 | Format-stamp admission guard (`🧷️format-stamps`) on every EXCLUSIVE lease of `repository-graph-facts:`, `build-graph-state:`, `javascript-dependency-*`, `native-fixed-image:` | 6 declared stamps (graph.generation, graph.record, graph.inferrers, dependency.walk, cache.graph-image, native.publication) in the schemas on disk vs the ones the process loaded | an old implementation must not overwrite an artifact written under a newer format | the command; the holder yields without producing. Message names the stamp, old and new value (EN+DE). Correct and rare (hand-bumped). |
| 3 | Lease yield handler (same file) | a holder is asked to give up a resource, or is idle for the declared time | queued exclusive peers | the command |
| 4 | `requireCurrent*` self-checks (observation, inference reuse, source inventory, digest read, revision reader, custody consumer, cache store schema, closures, cargo unit descriptor/selection/publication/receipt) | sha of the module's own source vs load | a module must not run half old, half new | the command; SKIPPED while `implementationPinnedV1()` (every command kernel and the door run pinned; the resident service does not) |
| 5 | Half-edited module load (`BuildMessage`/`ResolveMessage`/`Export named`) and concurrent edits that escape every layer | the failure itself | a peer is mid-write | the command, after a back-off of 0.25 to 30 s (EN+DE line with reason) |
| 6 | Task yielded stale (semio backend, nx task exit 75 found in the output) | child exit 75 | a nested command asked for a restart | the outermost command |
| 7 | Graph queue yield (`🕸️graph/🕰️generations/🛌️yield`) | long wait for a peer's graph build | do not hold a slot | not counted; the door waits alone, then starts again |
| 8 | Slice worker exit 75 (`🏡️resident/…/👷️slice-pool`) | the worker's stale exit | N7: restarted up to 3 times, then refused with `slice-restarts` ("later") | the load in flight (the plan phase again); published waves stay |
| 9 | Resident service identity (N7, `HostPolicy.serviceRevisionFiles`) | digest of the service files | a service on old code must not serve | the resident in-memory graph, nothing else |
| 10 | Nx plugin inference cache key `implementationRevision()` (`🕸️graph/📥️inference/♻️revision`) | whole bytes of ALL inference implementation sources | a cached answer must match the code | every cached nx `createNodes` answer on any byte edit of ~100s of files (not a restart; a recompute) |

There is no whole-closure-hash restart in live mode; the sealed mode (`SEMIO_REPO_SEAL=sealed`, off) has its own snapshot identity. I could not map "a 37 minute build restarted when a folder changed" on this server to a trigger: candidates are 2 (stamp moved), or the plan going cold (`changes===undefined` -> `cold("frontier")`, `manifest.identity!==identity`) after the folder change.

## What was built (scratch, landing script ready)

Schema first: NEW `LIB/🏗️build/📥️source/🔏️closure/🔁️dependencies/🧬️schema/🔣️.json` (`Protocol`: contract suffixes, listed-file bound 4, probe timeout 120 s and 1 MiB, EN+DE texts `restart`, `adopted`, `probeFailed`, `handoff`, `more`; `Case`).
- `🔁️dependencies/🟦️.ts`: `projectionAdmissionActionV1`, `projectionAdmissionV1` (gate: `admit()`, `identity()`, `adopted()`), `dependencyReadmissionV1(files)` now names the files in EN+DE, `withDependencyReadmissionV1` text is bilingual and says what happened (Nx step asked for a fresh admission, exit 76) instead of "dependency publication".
- `LIB/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs` (+ `.d.mts`): `nxBuildDependencySourceRevisionsV1`, `nxBuildDependencySourceChangesFromV1`, `nxBuildDependencySourceProbeV1` (fresh `bun --eval` child, bounded time and output, any failure is an error); `LIB/🟨️.mjs` (+ `.d.mts`): three wrappers.
- `LIB/⚡️caching/🚀️bootstrap/📜️script.ts`: 2 lines (`admission=projectionAdmissionV1(projection)`, `implementationIdentity=admission.identity()`); `launch` keeps calling `admitProjection()` before every spawn, now against the adopted baseline.
- Files in `T/🧰️scripts/P1/`: `payload.tar` (mirror + base), `land.py` (`--dry-run` ran green against the live tree: 13 files, 4 new; `--land` needs `N/LAND-OK`, takes `T/landing.lock` with an owner file, three-way merges with `git merge-file` on plain files, runs the 6 test files, appends to `T/landings.md`, adds an EN+DE entry to `📓️build-guide.md`).

## Tests (all run in scratch against the live modules; counts)

- NEW `🔁️dependencies/🧪️tests/🛂️admission` 10: closed schema, action table equal to an independent SQL oracle (bun:sqlite), no probe when unchanged, adoption by one fresh process with a bilingual announcement, the invocation language selects one language, restart after the identity was handed out, contract file restarts without a probe, failing or still-moving fresh process restarts, long lists ("a, b, c, d, and 2 more" / "und 2 weitere"), bilingual handoff text.
- CHANGED `🔏️implementation/🧪️tests` 1 (+ fixture + schema, 6 cases, SQL oracle for readmitted, probes and identity calls): runs the real `admitProjection` statements extracted from script.ts by AST.
- NEW `🔏️implementation/🧪️tests/🌱️probe` 4: a REAL bun child loads a fixture module and its report is read back; child that throws, times out, answers garbage or too much; command, env and bounds.
- CHANGED `🕰️generations/🧪️tests/🌊️waves` 7 (1 new): see verdict 4.
- Unchanged and green with the new module: `🔁️dependencies/🧪️tests` 5 (incl. the real Original retire/relaunch laws).
- Measured: two fresh bun processes evaluate the live projection identity in 40 to 60 ms and agree (`7f2dd842…`, changes 0).

## What a worker will see (after landing)

If code files of the projection moved while it planned: instead of the restart lines, `The sources of the Nx projection changed while this command was planning (<files>). Only code moved, so a fresh process evaluated the current sources and this command goes on with them: no restart, nothing planned again.` and the German twin. If a `*.json` contract moved: `The sources of the Nx projection changed while this command was running (<files>). A contract file moved or the change cannot be adopted, so the command starts again with the current implementation.` then the unchanged `Restarting with the current implementation (n of 6)`.

## Why "owners 0 of 703" (candidates, code facts)

1. The wave checkpoint covers only DIRTY owners. In `buildOnce` (`🕸️graph/🕰️generations/🟨️.mjs`) `wave=names.slice(0,ownerWave)` slices `found.dirty`; `discover:found.added` (owners new to the generation) and a cold start are ONE compute that publishes only at its end. A process that dies or restarts during it loses the whole compute; only per-project records survive. The log's "of 703" (a cold build would print 0) says the 703 were stale or added, not that the store was cold. NOT fixed: publishing a partial generation without a record of the owners still to discover would make the next membership diff report "clean" with owners missing; it needs a manifest field (`pending`), i.e. a graph.generation stamp bump. Recommended as its own package.
2. Per-project records hold only while every observed row holds, module-load rows included. N9 moved 20 whole-file rows under stamps (landed ~12:15Z, after that round); any other loaded file outside a stamped fragment still restales ALL records on a body edit. Not enumerated live.
3. A moved format stamp or `manifest.identity` makes the plan cold.
Decide with data: the `graph:` lines of the worker's resident service log (`$TMPDIR/semio-repo/<uid>/host-*.log`, stderr of the slice worker is inherited there): `graph: no usable generation (<reason>)` = cold (3), `graph: refreshing N projects`, `graph: R stale facts (kinds) leave N projects stale` and the first `graph: <project> is stale: <why>` lines name the changed file (2). Next round: collect them from every worker.

## Not verified

- Nothing landed; no live `repo` run; the real probe against the landed root (needs the new exports), the native-caller neighbour test (needs `nx` resolvable, fails in scratch only for that reason) and the whole path "Original adopts, Nx process accepts the identity" on a real worker.
- Risk accepted: the parent keeps its OLD loaded code for everything except the identity token; the Nx plugin reads the graph through the format-stamped contracts, so a code-only change that alters semantics without touching a contract file would be seen only by the Nx process. The Nx process rejects an identity mismatch, and every launch re-checks against the adopted baseline.
- Triggers 2 to 10 are described, not narrowed; the slice pool still cannot say which file made its worker exit 75 (the worker's stale message goes to the service log, not to the progress line).
- Cold and added-owner checkpointing (candidate 1) is only analysed.
