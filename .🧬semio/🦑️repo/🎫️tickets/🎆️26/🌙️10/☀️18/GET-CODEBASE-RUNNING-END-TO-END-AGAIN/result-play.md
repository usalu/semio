# Result: semio-tech play stale references (executor `play`)

Static only. No builds, no activation, no serve. Nothing committed or staged.

## Findings that changed the plan

- `bun repo` has no `components` command. `ComponentsScript` in `🚀️release/🟦️.ts` is not registered by `RepoScript`. The old per-plugin `@semio-tech/<p>-plugin:describe` therefore has no direct successor.
- The current per-plugin route is the generated `component` target of each plugin/extension crate: `bun repo run <packageName>:component`. The target builds, describes, receipts and materializes one component (inference `🟨️.mjs` line 1222). `packageName` is the `packageName` column of `PLUGIN_BUILD_TARGETS` / `EXTENSION_TARGETS`. It is `semio-hub-<id>-component` for the 44 plugins, but `semio-s-plugin-<family>-<name>-component` for 26 extensions/modules, so a plugin id cannot be substituted blindly.
- `@semio-tech/os-plugin-describe-rs:describe` is only the descriptor tool's CLI passthrough. The describe prerequisite of every lane is `@semio-tech/os-plugin-describe-rs:build`. That is already a dependency of each `prepare-<lane>-react-prototype`, hence of `activate-prototype`.
- Scripts that call `receiveScriptProcessInvocation` throw "Original parent Script process invocation required" when started as bare `bun script.ts`. They must run through `bun repo run` or `bun repo exec`. A play `📜️script.ts` command that shells out to `bun repo run` would be a nested backend invocation, which `playfreshbuild` forbids for the site script. I therefore did NOT add a sequencing command to the play `📜️script.ts`. The sequences are launch entries plus the graph.
- `components:complete` is not a missing Nx target. It is an in-process completion group in `native-build/🧩️component-pass/🟦️.ts` (lines 407-408: `/^components:(.+)$/` resolves when every `<component>:<name>` completion resolves). `🧪️release-job-order` pins it. No change needed.
- Lane/pane counts: `🧪️playactivation` already derives lanes from `playActivationLanes()` and compares them with the `project.json` dependsOn. No test asserts 35 or 149. Only prose counts existed.

## Changes

Play project (`🏢️semio-tech/🎡️play`), wording only, no behaviour change:
- `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`: header "Nx prepares" -> "The build tool prepares"; error text -> `Run the play test-e2e target: bun repo run @semio-tech/semio-tech-play:test-e2e`.
- `🔨️modules/🧪️e2e/📜️script.ts`: two docstrings, "owned by Nx" / "original Nx service" -> build-tool wording.
- `🔨️modules/🧩️runtime/📜️script.ts`: three docstrings ("outer Nx preparation graph", "its Nx prerequisites", "lifetime owned by Nx").
- `🔨️modules/🧩️runtime/🟦️.ts`: docstring "resolving 31 plugin descriptors" -> "every plugin descriptor".
- `🧪️tests/🧪️playpanedefaults/🟦️.ts`: two docstrings with "58 panes share 31 plugins" / "31 plugin descriptors" made count-free.
- `🧪️tests/🧪️playactivation/🟦️.ts`: test title "Nx graph" -> "project graph".

Launch (both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, same position after `🛠️component🏢️semio-tech🎡️play🎪️demonstrator`, group `4_build`, order 11.041):
- New entry `🛠️component🏢️semio-tech🎡️play🔌️plugin`: `bun repo run ${input:playComponentPackage}:component --skip-cache --skip-remote-cache`.
- New input `playComponentPackage` (promptString, EN/DE description) appended to `inputs`.
- `launch.json` re-parsed with `json.loads`: valid, 13582 configurations. The seed is JSONC and was edited with the same anchored insert, not parsed.

Ticket scripts (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/`):
- Deleted `📜️describe-all-then-activate.sh`, `📜️describe-plugins.sh`, `📜️describe-serial-then-activate.sh`. They called `bun nx run …:activate-dev` and `@semio-tech/<p>-plugin:describe` (both gone), hard-coded `/Users/ueli/...`, and wrapped a wasm mutex that the coordinator now replaces.
  - Replacement for "describe all, then activate": launch `♻️activate🏢️semio-tech🎡️play` (`activate-prototype`, whose graph already depends on describe-rs:build and every lane).
  - Replacement for "describe plugin X": the new component launch entry.
- `🔁️serve-supervisor.sh`: start line was `bun ./🔨️modules/🧩️runtime/📜️script.ts serve` (bare launch, now refused) with hard-coded ROOT. Now `bun repo run @semio-tech/semio-tech-play:serve --exclude-dependencies` from a `git rev-parse --show-toplevel` root; `DEVELOPER_DIR` pin dropped.
- `📓️play-runtime.md` line 73: `bun nx run …:test-e2e` -> `bun repo run …:test-e2e`.

## Verification

- `grep -rniE "\bnx\b"` over the play tree (excluding dist/node_modules): remaining hits are the framework import path `🔌️nx/📦️outputs/📚️cohorts` in `📄pages/📦️publication/🟦️.ts` and the negative assertion in `🧪️playfreshbuild`. Both are correct as is.
- `grep` for `activate-dev|activation/dev|bun nx|nx run|-plugin:describe` in play, `.vscode/launch.json` and the seed: no hits.
- `git diff` of the play tree shows only the wording changes above for my files (other hunks are peers').
- Not run: any test, build or launch entry.

## Still open

- `project.json` keeps the explicit 35-lane dependsOn lists. They are pinned to `playActivationLanes()` by `🧪️playactivation`, so drift fails the test. Deriving them in inference would couple the graph inference to the play runtime; not done.
- Other fleet-era shell scripts remain in the old ticket folder (`📜️wasm-build-mutex-*.sh`, `📜️native-test-mutex.sh`, `🧪️fleet-wait.sh`, `🧪️test-binary-watchdog.sh`, `📜️daemonize.py`, `🔁️serve-supervisor.sh`). They are temporary ticket tooling, not stale `nx` callers, but `AGENTS.md` forbids shell scripts as permanent tooling. The supervisor in particular needs a TS port if it is to live on; it cannot be a bare `bun script.ts` (invocation envelope) and must not nest `bun repo run`, so it would have to run under `bun repo exec`.
- Dated logs in that ticket (`📓️status.md`, `📋️fleet-brief-v5.md`, etc.) still mention `activate-dev`, `describe-*.sh` and "28 lanes". They are history and were left alone.
- User memory `project-semio-tech-play-boot-recipe` and its siblings still carry `activate-dev`, `dist/♻️activation/dev`, 26/28 lanes and the deleted scripts. Not edited here.
- Launch `🛠️dev🏢️semio-tech🎡️play` sets `SEMIO_RENDERER=react`, which play code never reads. Left as is.
- `package.json` scripts of play call `bun --cwd "../.." repo run …`, `AGENTS.md` says they should call `nx`. Unchanged; needs a decision.
- Unverified at runtime: whether `activate-prototype` refreshes committed descriptors by itself (the `component` target does; the lane graph compiles components inside its own tasks).
