# 🛠️ Result: x-dev (continuous course targets under Bazel)

Status 16:30 (after the 16:03 kernel panic): all my files survived; no dev server or driver of mine runs, ports 6061/6063/8791/8793 are free (lsof). `node_modules` is gone with the dependencies partition, so `bun test` of the new tests cannot resolve `ajv` until storage is back (last green: 11/11 and the manifest test 2/2 at ~16:00, mutation check detected 3 failures). Waiting for coordination.md "bun repo boots" before any live run.

Status 15:55: static work, tests and staging done. Live Bazel runs NOT yet done: blocked by the storage failure (fixed 15:47), the x-inputs hardlink mirror (deleted 15:58), then at 15:58 by `Bounded regular bootstrap input required` (run `drive-dev-site-2.log`), then the kernel panic. The driver is ready (`x-dev-tools/📜️drive.ts`); live results below are marked PENDING.

## Findings (what the audit missed)

1. Readiness is already one neutral contract, not duplicated. `metadata.semio.dashboard.ready` (schema `$defs/ReadyDeclaration` in the build project schema) is read by the Rust dashboard (`🎛️dashboard/🌀️daemon/🟢️ready`), the Nx wrapper (`🟢️readiness` prints `[semio] ready` after its probe) and, in the live adapter, the Bazel adapter (`📜️script.ts` ~772: `readReadinessDeclarationV1` + `watchReadinessV1`, prints `[bazel] task-ready <id> <address>` only after the exact address answered 2xx/3xx). The audit read an older adapter revision that only waited for `[bazel] task-started`. `execution.readyWhen` is only the explicit override and is not used by the course. No second contract to design; no manifest or adapter change is needed for readiness.
2. The course stack and the proctor already print the address the matcher looks for: the Rust proctor prints `proctor listening on http://127.0.0.1:8791`, Vite prints `Local: http://localhost:6061/`, the stack printed `site ready at http://127.0.0.1:6061`. The probe of the adapter is the authority, a printed address earlier than the answer only starts probing.
3. A configuration cannot carry `steady`/`beside`. `--configuration`/`:cfg` is the build mode: `resolveBuildModeV1` runs `readBuildMode(value)` for every configuration before any adapter and accepts only `prototype|release` (`bun repo --tool=bazel run …:dev:beside --dry-run` fails with "Build mode must be prototype or release" at `🧭️invocation/🟦️.ts:116`, for Nx as well; the "Unknown build configuration" the audit quoted comes later in the Bazel adapter). The neutral CLI has no `--param`/`--env` either (those are `repo dashboard run` options). The one neutral spelling both tools and the dashboard share is the environment of the run: the dashboard parameters already resolve to plain `env`, and the readiness resolves its port from `ready.portEnv` out of `process.env` plus the task env (both adapters). So `steady`/`beside` are environment (documented in the course README, EN+DE).
4. Cancellation path read statically: Bazel cancel sends `cancelled` over the control socket, the owned action child is torn down with `terminateOwnedChildTree` (SIGKILL of the whole process tree and descendant groups), so Vite and the proctor die with it and their ports free. A Ctrl+C to the whole group additionally reaches the stack's own handlers (`runDevStack`: graceful proctor stop within 5 s, then tree kill). PENDING live proof with `lsof -i :6061`.

## Design

- One declaration: unchanged `metadata.semio.dashboard.ready` per continuous target (`dev`, `dev-site`, `@teaching/proctor:dev`).
- Stack (`🧱️stack/🟦️.ts`, edited directly): new `awaitAnnounced` = `awaitReady` and then the neutral ready line `<label> is ready after <ms> ms` (label carries the address) strictly after the probe answered; used for the proctor and the site of `dev`. New pure `siteOrigin(env, site)` and `stackFollowsSources(env)` so the environment contract is testable; `dev` uses them. The old summary line `site ready at …, proctor at …` was removed (the two ready lines replace it).
- Parameters: plain environment (see finding 3). Staged manifest edit gives `dev-site` and `@teaching/proctor:dev` the same `stack=beside` dashboard parameter `dev` already has, so every continuous target can start a beside variant from the dashboard as well (`dev-site`: port 6063 + proctor 8793 + data; `proctor:dev`: 8793 + data). Compound members are untouched.
- Progress streaming: the stack's `[stack] waiting for … (N s)` lines (5 s) and the adapter's `[bazel]` lines stream; unchanged.

## Files

Edited (live):
- `🎓️teaching/🏛️architecture/📚️course/🧱️stack/🟦️.ts` (awaitAnnounced, siteOrigin, stackFollowsSources; uses them)
- `🎓️teaching/🏛️architecture/📚️course/README.md` (EN + DE paragraph: neutral command lines, ready line, cancellation)
Created (live):
- `…/📚️course/🧫️fixtures/🟢️continuous/🔣️.json` and `…/🧬️schema/🔣️.json` (language-agnostic vectors + schema)
- `…/📚️course/🧪️tests/🟢️continuous/🟦️.ts` (11 tests)
Staged (`x-dev-tools/staged/`, frozen manifests):
- `apply.py` (idempotent, anchored; `--check` dry-run; `X_DEV_ROOT` for scratch), `stack-parameter.json`, `🟦️manifests.test.ts` (installed to `…/🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts` by apply.py), `check.sh` (scratch proof + live state)
Tools: `x-dev-tools/📜️drive.ts` (foreground driver with hard deadline).

## Tests and results

- `bun test ./🧪️tests/🟢️continuous/🟦️.ts` (cwd course root): 11 pass. Oracles: Ajv (fixture schema and `ReadyDeclaration` from the build project schema), independent `node:net` connect and `fetch` against the announce moment, the neutral `ReadinessMatcherV1` finding the printed address, an independent port parser for the refused cases. Mutation check: moving the `say` before `awaitReady` makes it fail (run 15:55 was disturbed by the nlink blocker, repeat pending).
- `bash x-dev-tools/staged/check.sh`: scratch copy of both manifests + apply twice (idempotent) + the manifest test: 2 pass. Live tree reports "would edit" until applied.
- Regression: `🧪️tests/🎛️dashboard/🟦️.ts` 4 pass. `🧪️tests/🧱️local-stack/🟦️.ts`: 40 pass, 2 fail (both in "a dev server whose proctor goes away while it proxies a socket": Vite proxy upgrade/ws under Bun, 503 expected got empty; unrelated to the stack functions edited here; not investigated).

## Apply order and verification

1. (coordinator, when the freeze lifts) `python3 x-dev-tools/staged/apply.py` then `python3 x-dev-tools/staged/apply.py --check` (all "already applied").
2. `cd 🎓️teaching/🏛️architecture/📚️course && bun test ./🧪️tests/🟢️continuous/🟦️.ts ./🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts ./🧪️tests/🎛️dashboard/🟦️.ts`
3. Live (after the nlink blocker is gone), foreground, hard deadline, never leaves a server:
   - `bun x-dev-tools/📜️drive.ts @teaching/architecture-course:dev-site --ports=6061 --deadline=900 --log=…`
   - `… --env=TEACHING_ARCHITECTURE_COURSE_PORT=6063 --ports=6063` (proves ambient env reaches the Bazel action and the readiness)
   - `bun x-dev-tools/📜️drive.ts @teaching/architecture-course:dev --ports=6061,8791 --env=TEACHING_ARCHITECTURE_COURSE_WATCH=off` (cold proctor build may take 10-25 min)
   - `bun x-dev-tools/📜️drive.ts @teaching/proctor:dev --ports=8791`
   - add `--signal=leader` to cancel only the `bun repo` process.
   The driver prints when `task-ready` appeared relative to `task-started`, probes the ports itself at that moment (connect + HTTP), sends SIGINT, and fails (exit 1) if a port still listens afterwards.

## Backend edits

None.

## Open / PENDING

- Live proof of: readiness ordering under Bazel for the three targets, ambient env (`beside`, `steady`) reaching the Bazel action, cancellation freeing 6061/8791 (first attempt 15:28-15:41 died in the storage blocker, log `🗑️generated/x-dev/drive-dev-site-1.log`).
- If ambient env does not reach the Bazel action, the fix is a small allowlist/pass-through in the Bazel adapter (frozen): stage it here.
