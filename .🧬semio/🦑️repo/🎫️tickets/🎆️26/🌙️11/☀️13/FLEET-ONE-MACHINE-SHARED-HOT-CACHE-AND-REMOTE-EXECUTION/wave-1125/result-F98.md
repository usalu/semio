# F98: typecheck program boundaries, NODE_ENV class 372 -> 0

Lane F98, 2026-11-26. Product level `repo typecheck --all-errors --no-importers 🧰️framework/🛍️products/🦑️repo` (5177 files): 372 errors before, **93 after**. All 88 + 78 + implicit-any errors of the Next.js `NODE_ENV` class are gone (0 mentions of `NODE_ENV`); the 93 left are ordinary, unrelated type errors that the old noise sat beside (not caused by program composition).

## Cause (confirmed, one step further than F95)
`repo typecheck` composes one compiler program per policy project with `files` = the touched files, extending the project's tsconfig. Files outside the `library` roots all went to `workspace`. That put the Next.js route modules and, more importantly, the non-Next server tooling (coordinator builder, package scripts and tests, which import the build library) into one program with `next-env.d.ts` (matched by `**/*.ts`) and the `next` imports. `next/types/global.d.ts` then made `NODE_ENV` required on `ProcessEnv` for every `spawn`/`env` of the build library in that program.

## Fix (schema-first, policy-driven)
- `library/🏗️build/🔬️typecheck/🔣️.json` + `🧬️schema/🔣️.json` (new required `ambient` list, definition `Ambient {id, declaration, projects}`): two new projects before the fallback, `server-next` (the Next route modules, `📚️library` of the server, the app folder and `next-env.d.ts`; tsconfig `🖥️server/tsconfig.next.json`) and `server-build` (everything else under `🖥️server`: builder, scripts, tests; tsconfig `🖥️server/tsconfig.json`, excludes the Next files and `next-env.d.ts`). `ambient`: `next` -> declaration `node_modules/next/types/global.d.ts`, allowed only in `server-next`.
- `typecheckProjectOfV1` (`🔬️typecheck/🟦️.ts`): a root also matches a file exactly (`file===root||startsWith(root+"/")`), so single files can be roots.
- Product tsconfig: `🦑️repo/tsconfig.json` drops the one `files` entry and excludes `🔨️modules/🖥️server/**` (the server project tsconfigs extend it and carry its `@/lib/*` paths). `skipDirectories` gains `📁️some` (the lint fixtures that the tsconfig already excludes; 6 false errors).
- Real bugs found and fixed on the way: `runCanonicalGoBuild` call without `invocation` in the three `🔬️go-package-commands` tests (`🖥️server/🎛️coordinator`, `🏃️test-runner`, `📊️metrics`).

## Law (recurrence guard)
`🔬️typecheck/🧪️tests/🟦️.ts`, two tests: (1) the policy: every ambient declaration exists, names known projects, `next` is allowed only in `server-next`, routing of Next files / builder / script / server tests; (2) AMBIENT LAW programs: composes the real programs of `repo typecheck` for the product (`planTypecheckV1` groups + `typecheckConfigurationV1`), lists each program's files with one `node tsc --listFilesOnly`, and fails if any program not allowed for a declared ambient package reads its declaration (checks the library, the CLI `workspace`, `server-build`) and fails if the allowed program does not (positive control, so a stale path cannot pass silently). 22 pass. The test needs ~3 GB with its compiler child: entry added to `🏃️process/🧪️testing/👷️workers/📏️file-budgets/🔣️.json` (3600 MB, sorted by UTF-16 path).

## Counts
| Check | Before | After |
|---|---|---|
| product level errors | 372 | **93** |
| of them `NODE_ENV` class | ~250 | 0 |
| `🖥️server` folder (projects `server-next` 30 files, `server-build` 35 files) | 246 (first split attempt: library files seen under NODE_ENV) -> 225 | **0** (passed, 36 s) |
| declaration drifts | 0 | 0 |
| typecheck module + go-package-commands tests (touched) | | passed |
| `repo test-file` typecheck tests | 21/1 (new law) | **22/0** |
| file-budgets tests | | 8/0 |
| `dangling-imports.py` (typecheck, server, test-runner/metrics tests, file-budgets) | | exit 0 |

The Next.js project's own program (`server-next`, 30 files incl. `next-env.d.ts`) typechecks with 0 errors; it is the only program that reads `next/types/global.d.ts`.

## The 93 left (not done, not a program-boundary problem)
89 are in `🤝️shared-worktree` (replica, attest 23, provision 7, trust 7, git 6+3, resilience 9, canary 4, clash 4 ...: `expect` typing against unions, missing `.d.mts` for `restart-in-place`, `renameSync` import in `🗺️topology/🧪️tests/🔵️.ts`, `child.stdout` possibly null in `🧲️adopt/🟦️.ts` and `🥾️bootstrap/🟦️.ts` product code of the live replica daemon), plus `🚀️launcher` 2, `🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain` 2, `🔎️search/🧪️tests/🧵️shards` 1, `🧪️test/🧪️tests/🧬️schema-invariants` 1, `🚧️boundary/🔤️fold/📜️script.ts` 2. I did not touch them: the replica daemon restarts on code change and the area is edited by other lanes. A follow-up lane can do them with the F95 method (`expect<unknown>`, typed rows).

## Notes
- First full run of the product after a policy change is cold for the library program (4686 files): it hit the 600 s budget twice before the incremental state was saved; after one finished run it takes 150 s.
- `repo typecheck` runs its compiler under bun (4.3 GB RSS for the workspace program); the same program under node is 2.3 GB. Not changed here.
- Landing lock `build` taken before the edits, released after the checks.
