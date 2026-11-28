# Result H30: single-project `repo run` and `repo inputs` on the coordinator

Project used (same as H18): `@semio-tech/repo-workspace-go:generate-source-policy:prototype`. Default tool is Nx (the error path runs through the Nx backend).

## Root causes
1. **`Actual declared requested Source scope required (7566ad79)`**: `ensureBuildScopesV1` (`🏗️build/🚀️bootstrap/🟦️.ts`, called by the Nx and Bazel backends for every run) passed the member packages of the root `package.json` `workspaces` as the "declared scopes"; each project root became its own owner (e.g. `🧰️framework/🛍️products/🦑️repo`), which the physical Source-owner model (6 scopes: `.`, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`; checked with `withCurrentBunWorkspaceOwnershipV1`) does not contain. It only surfaced once the sync receipt was stale (`env:BUN_ENABLE_CRASH_REPORTING`), because a current receipt short-circuits before the owner check. Reproduced here (run1, 17 min, exit 1). **Fixed by H26 at 02:41** (`javascriptDeclaredScopesV1`, `T/landings.md`); I found the same cause independently, dropped my duplicate patch (no change from me in that file).
2. **`repo inputs` read the whole workspace**: `buildInvocationGraphSelectionV1` returned `{kind:"all"}` for every command except `run`, although `inputs` plans exactly one task with `excludeDependencies:true`. **Fixed by me** (landing "Wave 1127 H30", 03:05, lock graph): `inputs` now selects `{kind:"tasks",excludeDependencies:true}` like `run --exclude-dependencies`. Test `🔎️show-project-scope` 4/0, admission suite 13/0, typecheck 58 files 0 errors, dangling-imports 0.
3. **`tool-closures-refused` (24, ui-rs:generate)** is not reproducible now: the all-closure preflight (`preflightToolClosuresInChildV1`) passes, 40 checked, but it takes 186 s on this loaded host and runs once per process in every graph build, narrow or whole (`🕸️graph/🕰️generations/🟨️.mjs` `prime`). The refusal H18 saw was a declared tool closure importing generated output at that moment (policy text `closure.tool-closures-refused`); transient, no code fix.
4. **15 min in the graph read**: the narrow read (`tasks` selection, `excludeDependencies`) is correct and small, but with policy `fresh` it waits for the builder custody (`exclusive graph-custody`) held by the coordinator's rolling background whole reads (`show projects`, restarted after every landing). run1 waited 280 s behind one ("waiting for the graph state lease", 1 of 600), then more. NOT fixed: needs the builder to serve a narrow request first or the narrow read to use a sealed generation; see Open.

## Measured (coordinator, load 50-60, nice -n 10)
- `repo inputs <project>:<target>`: exit 0, prints JSON (project, target, files, ...), total 22:01 (graph slice 254 s, parser Source listing ~4 min, Nx tool verification 11k entries ~5 min, Nx project graph). Before: whole read + preflight failure.
- `repo run <project>:<target> --exclude-dependencies` (first, cold): exit 0, `Successfully ran target`, Nx `Cache 0/1 hit`, run 34 s, total 10:39 (graph slice ~200 s, Nx tool preparation).
- Same command again (run with `NX_CACHE_DIRECTORY` pointing to an empty scratch dir): 46.7 s total, `tasks: 0 stale, 1 up to date ... the build tool was not asked` (task-key receipt in `.cache/b/g/m/task-keys`).

## Remote-cache proof: NOT delivered
`repo remote-cache status` Nx counters, lifetime before (02:30): hit 4 / miss 417 / put 154 / rejected 45; after both runs (03:48): hit 4 / miss 484 / put 171 / rejected 50. Server restarted 02:36 (restarts 39); since then (session) hit 0 / miss 17 / put 0 / rejected 0, covering my run (03:31-03:42): the cold run missed (a lookup) but NO Nx put is visible, and the repeat never reached Nx because the repo receipt layer answers first. So: remote hit not shown, and no evidence that this machine uploads Nx results (token on this machine: `present:false`, route is the signed fleet-identity route; Nx prints nothing about the remote cache). A hit needs a different machine (or the receipt dropped) and a visible put first.

## Open / next
- Prove put+hit: run the same task on a worker (empty local Nx cache, no task-key receipt) and read the `nx` counters before/after; first check whether a put appears right after a cold run on any machine (session put 0 for 70 min is suspicious: Nx upload may be silently failing or the signed route not used for PUT).
- Narrow reads should not wait for the whole builder (graph custody) or should run the preflight once per generation, not once per process (186 s).
- `repo inputs` for Nx still pays Nx tool verification + Nx project graph (~15 min cold); the in-house engine path (`--tool=semio inputs`) already uses the narrow graph only and is the fast route; not measured.
- Unverified: `--tool=semio inputs`, any worker.

Files: `🏗️build/🕸️graph/🚪️admission/🟦️.ts`, test `🏗️build/🕸️graph/🚪️admission/🧪️tests/🔎️show-project-scope/🟦️.ts`, script `T/🧰️scripts/H30/apply.py`.
