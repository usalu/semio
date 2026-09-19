# Z1 — Zero-touch setup + launch rows (execution slice for G4)

Source audit: `📓️g4-zero-touch-and-run-paths.md`. Every claim below is either a captured command
output in `🗑️generated/z1-*` or a cited file:line.

## 0. G4's P0 #1 premise is wrong — measured, not argued

G4 concluded that every `⚛️react` "s" launch row silently serves wgpu, because the nx target
`@semio-tech/framework-os-dev:dev` hard-coded
`…/🧊️wgpu/🌐️server/📜️script.ts serve s dev`, which force-sets `SEMIO_RENDERER = "wgpu"`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:21`).

That reasoning misses the `bun nx` wrapper. Root `package.json:"nx"` maps `bun nx` to
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx`, whose
`resolveNxInvocation` (same file, lines 261–281) **rewrites** `@semio-tech/framework-os-dev:dev` /
`workspace:dev` into the renderer- and variant-specific target
`<dev|serve>-<variant>-<react|wgpu>-<dev|release>` the Nx plugin generates at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1020-1027`. The static `dev` target was never
reached through `bun nx`, so `SEMIO_RENDERER=react` was already honoured, and the wgpu server's
assignment only ever ran inside an already-wgpu-selected target.

Probe `🐍️z1-nx-invocation-probe.ts`, capture `🗑️generated/z1-nx-invocation-before.txt`:

| launch row | resolved nx target |
|---|---|
| `🛠️dev🪐️space⚛️react` (6070) | `@semio-tech/framework-os-dev:dev-s-react-dev` |
| `🛠️dev🪐️space🧊️wgpu🌐️wasm` (6071) | `…:dev-s-wgpu-dev` |
| `🛠️dev🖥️s👤️1⚛️react` (6072) / `👤️2` (6073) | `…:dev-s-react-dev` |
| `workspace:dev -- s served` | `…:serve-s-react-dev` |

`dev-s-react-dev` runs `bun ./📜️script.ts serve s react dev` → the React `ServeScript`
(`🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:45-56`) → Vite. So "the only working React route is
unregistered" is also wrong — it is the route all three react rows already took. **The same probe
found a real P0 that G4 missed (§1).**

## 1. P0 (real) — the `✏️draw` launch rows booted the `space` playground

`resolveNxInvocation` took the variant from the `--` segments and, when the alias
`@semio-tech/framework-os-dev:dev` carried none, fell back to the literal `["s"]`, **ignoring
`SEMIO_PLUGIN`**. Exactly two of 47 `devLaunchers` rows used that bare alias: `s` (where the `"s"`
default is accidentally right) and `draw`.

Measured pre-fix (`🗑️generated/z1-nx-invocation-before.txt`):

```
alias :dev + SEMIO_PLUGIN=draw (S_OS_PORT=6088)
  nx run @semio-tech/framework-os-dev:dev-s-react-dev
  plugin=s renderer=react port=6088
```

Both `✏️draw` rows served the space hub on draw's ports 6088/6089.

Fixes:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:272` — the bare
  alias now selects the variant from `SEMIO_PLUGIN` before falling back to `"s"`. The alias can no
  longer resolve to a different playground than its own row's env.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:127-135` — the
  static `dev` target no longer hard-codes the wgpu server script or the `s` variant, and no longer
  carries `dependsOn: ["…:activate-s-wgpu-dev"]`. It forwards to the root `dev` verb, which re-enters
  the wrapper and lands on the renderer/variant target. Bypassing `bun nx` (raw `npx nx`) can no
  longer silently serve wgpu on a react row.
- `.vscode/🧩️launch.seed.jsonc` — `devLaunchers.draw.command` / `.s.command` moved to the canonical
  `bun nx run workspace:dev -- <variant>`, so all 47 variants name their variant in the target.
  *(A peer's live refactor of `🚀️launch/🟦️.ts` has since made `command` generator-owned via
  `playgroundDevCommand(variant)` — the same shape, generalized; the seed's per-variant `command`
  field is gone. My change is absorbed by theirs and stays correct.)*
- Stale docstrings that produced G4's wrong conclusion: `📜️script.ts:267-271` and
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:2732-2735` now state that the target selects
  the renderer.

Proof after the fix — `🐍️z1-dev-target-assertions.ts`, capture
`🗑️generated/z1-dev-target-assertions.txt`, all `ok`:

```
ok  s/react    -> @semio-tech/framework-os-dev:dev-s-react-dev
ok  s/wgpu     -> @semio-tech/framework-os-dev:dev-s-wgpu-dev
ok  draw/react -> @semio-tech/framework-os-dev:dev-draw-react-dev
ok  draw/wgpu  -> @semio-tech/framework-os-dev:dev-draw-wgpu-dev
ok  @semio-tech/framework-os-dev:dev is renderer-neutral
```

## 2. P0 — launch-name collisions

Making `s` canonical exposed a second defect: `normalizeDevLaunchConfigurationNames` rebuilt each
`3_dev` name as `🛠️dev<taxonomy prefix><renderer marker>` and **dropped the `👤️N` user slot**, so the
two collaboration rows collapsed onto the single-user one. The same normalizer already collapsed 9
pairs at HEAD (cad/shooting fixture rows onto their plain siblings, plus duplicate native rows) —
verified against `git show HEAD:.vscode/launch.json`.

Fixes in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts`:
- `devLaunchNameSuffix` (line 96-105) keeps a trailing `👤️<digits>` before the renderer marker — the
  user slot is part of a row's identity exactly like the renderer is.
- `normalizeDevLaunchConfigurationNames` (line 124-142) skips a rename that would land on a name
  another row already carries: a distinguishable stale-emoji name beats two identical Run-panel rows.

Measured on the rendered output (`🐍️z1-launch-preview.ts`): **11 duplicate names → 0.**
A peer's in-flight rewrite of the launch law independently asserts `!/👤️\d+/u` on the pool it
filters, which confirms this is the intended shape.

## 3. P1 — launch rows: dead targets, missing lifecycle commands, broken compounds

`🐍️z1-launch-target-audit.ts` (capture `🗑️generated/z1-launch-target-audit.txt`) resolves every
`nx run <project>:<target>` in `.vscode/launch.json` against 434 static `📋️project.json` manifests,
the root script's `register(...)` command targets and the per-playground targets the Nx plugin
generates — closing G4's explicit "target level not verified" gap.

Before: **2 unresolved** of 115 pairs.
- `🛠️dev🧰️repo🔌️mcp🦀️rust` → `@semio-tech/repo-cli-rs:mcp`. That project declares
  `build/test/test-*/run/daemon/workflow`; the repo MCP moved to the Go `repo-mcp` project (M1). Row
  **removed** — the live route is the existing `🦑️mcp dev` row (`… dev mcp stdio client`).
- `🛠️dev🧰️repo🖥️coordinator🐹️go` → `…:run`. The project declares `build/dev/test`. Retargeted to `:dev`.

Added rows (group `3_dev`, orders −49.9…−49.2, next to `⚙️setup` at −50, in README's canonical order):
`▶️start`, `🏭️generate`, `🧹lint`, `🎨format`, `🧪️test`, `📦️build`, `🚢️publish`, `🗑️purge` — the eight
canonical root commands that had no IDE-launchable path at all (AGENTS.md line 50).

Added `🛠️dev🪐️space⚛️react📦️served` → `bun nx run workspace:dev -- s served`
(`serve-s-react-dev`): the no-rebuild React boot that skips the activation chain and the shared Cargo
lock. Renamed the 3_dev `🛠️dev♻️mit-bestand📋️zwischenbericht` row (which runs the generic
`mit-bestand-bericht:watch`) to `🛠️dev♻️mit-bestand📚️bericht`, resolving its collision with the 0_dev
row that runs `watch-zwischenbericht`.

**All three compounds were broken.** `🧭️compound🖥️s⚛️react🌉️os-mcp` and `…🖥️s⚛️react🗄️os-hub` referenced
`🛠️dev🖥️s⚛️react`, a name that has not existed since the normalizer started rewriting prefixes — both
already dead at HEAD, i.e. the "open the hub next to the MCP gateway" and "hub + frontend" one-click
paths never worked. The two-user collaboration compound broke under §2's rename. All member lists
updated to `🛠️dev🪐️space⚛️react` / `🛠️dev🪐️space👤️1⚛️react` / `👤️2⚛️react`.

After regenerating with `NX_DAEMON=false bun nx run @semio-tech/plugin-registry:generate` (exit 0):
**327 configurations, 118 project:target pairs, 0 unresolved, 0 duplicate names, 3/3 compounds
resolve**, and `🐍️z1-launch-preview.ts` reports the committed `.vscode/launch.json` byte-identical to
a fresh render (no hand edits).

## 4. P0 — devcontainer zero-touch

`postCreateCommand` ran only `workspace:deps-javascript`
(`.devcontainer/devcontainer.json:37`) → now `["bun","nx","run","workspace:setup"]`.

`📋️project.json:"setup".dependsOn` gained `setup-git`, `prepare`, `repo-mcp:build` and
`@semio-tech/framework-os-mcp-rs:build` alongside the nine `deps-*`. That closes, in one target, every
gap G4 listed: cargo fetch, go mod download, playwright chromium, cargo-nextest/llvm-cov, **both**
`rust-toolchain.toml` wasm targets (`deps-wasm` → `wasm32-wasip2`, its `deps-trunk` dependency →
`wasm32-unknown-unknown`), the generated schema/tokens/assets/graph/plugin-registry that every `dev`
route resolves against, the agent-instruction aliases, and the two MCP binaries `.mcp.json` launches.

`.devcontainer/devcontainer.json:25-27` — the Rust feature pinned a *second*, stable `1.92` toolchain
with only one of the two targets, which rustup overrides on first use inside the repo anyway. Set to
`"version": "none"`: rustup is installed, `rust-toolchain.toml` is the single source of truth, and the
pinned nightly + both targets are installed by `deps-cargo`/`deps-wasm` during post-create.

**Proof (task graph, no builds run):** `NX_DAEMON=false bun nx run workspace:setup --graph=stdout`,
exit 0, capture `🗑️generated/z1-setup-graph.txt` — **27 tasks, no cycle**, including
`workspace:deps-{javascript,python,cargo,go,dotnet,cpp,browsers,wasm,wasm-opt,trunk,tools}`,
`workspace:setup-git`, `workspace:prepare`, `@semio-tech/{framework-schema,ui-styling-tokens,
framework-graph,plugin-registry,ui-rs}:generate`, `@semio-tech/assets:build`, `repo-mcp:build`,
`@semio-tech/framework-os-mcp-rs:build`.

**Idempotence / cross-platform** (read, not run — see §7):
`prepareDependencies` (`…/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts:14-56`) probes before every
install (`tool()` compares `--version`, `target()` compares `rustup target list --installed`), spawns
with no shell and uses no bash-only construct. `runGit` (`📜️script.ts:360-380`) removes then recreates
each alias and falls back from `symlinkSync` to `linkSync` on `win32`;
`installMicroCommitGitHooks` (`📚️library/🟦️.ts:5242-5259`) overwrites its hook bodies and swallows
`chmodSync` on Windows. `prepare` and the two binary builds are Nx-cached.

**Root symlinks** — the task mentioned three `.generation3d-*-link`; there are **two**
(`.generation3d-crate-link`, `.generation3d-edit-link`), both tracked in git since 2026-09-09
(`599a5d8450`), both pointing into `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`. A repo-wide
grep finds **zero** references outside G4's own memo — nothing in any script, project.json, Cargo
manifest or source reads them. They look like an editor-navigation convenience. **Not deleted**, per
the slice instruction: reported for their owner to remove.

## 5. P1 — hard-coded absolute paths / macOS-only commands in the golden paths

Swept and found **clean**; G4 listed none concretely and I could not manufacture any:

- All 436 real `📋️project.json` target commands: **1** shell-ism (`&&`), and it is in
  `storybook-static/` (a build output, not source). No `nohup`, `rm -rf`, `cp -r`, `chmod`, `ln -s`,
  `lsof`, pipes, backticks or `$(...)` in any real target.
- The four golden-path chains (`📜️script.ts`, `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`,
  `⚡️caching/🚀️bootstrap/📜️script.ts`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `🌉️mcp/🟦️.ts`,
  `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`, `🤝️collaboration/🟦️.ts`): the only platform-specific call is
  `🌎️hub/…/📜️script.ts:611`, already branched `darwin → open`, `win32 → cmd.exe`, else `xdg-open`.
  `NativeOsScript` (`📜️script.ts:283-308`) is the documented PowerShell/sh bootstrap split.
  Port probing uses Node's `net` module, not `lsof`/`netstat`.
- Repo-wide `/Users/ueli/...`: the only hits are `recordedRepositoryRoot` values inside
  `📚️library/🔣️taxonomy.json` and the `🏺️historical-package-owner-identity` fixture — historical
  *data* records, not run paths, and owned by the taxonomy slice. Left alone; flagged here.

## 6. P1 — README and AGENTS.md surfacing

`README.md`:
- Step 4 now states exactly what container creation runs, and that every step probes before installing.
- The "Rust 1.92" line is replaced: rustup with **no** image-pinned toolchain, `rust-toolchain.toml`
  as the single source of truth (`nightly-2026-07-07`, both wasm targets).
- New **"The four golden paths"** table mapping each outcome to its launch row(s) and the nx target it
  resolves to, plus one paragraph explaining that `SEMIO_RENDERER` in a row's `env` is what selects the
  renderer through the `bun nx` wrapper.
- The `setup git` paragraph was **false as written** ("also invoked from `npm run setup`"). It is now
  true (§4) and states precisely which clients need a link and which read `AGENTS.md` directly.
- The canonical-root-commands paragraph now notes each has a `3_dev` launch row, removing the
  README-vs-AGENTS.md contradiction G4 flagged.

`CLAUDE.md`/`GEMINI.md` absence: the repo's mechanism is `SetupScript.runGit`'s alias loop, which had
simply never run automatically. Fixed at the root by the `setup` → `setup-git` dependency (§4), and the
alias list is now a documented export, `AGENT_INSTRUCTION_ALIASES` (`📜️script.ts:318-323`), extended to
`.github/copilot-instructions.md`: `copilot-chat` is a declared client (AGENTS.md line 93) and
`github.copilot-chat` ships in the devcontainer, but Copilot reads only that path, so `AGENTS.md`
never reached it. `codex`, `cursor-chat`, `windsurf-chat`, `droid` and `kiro-cli` read `AGENTS.md`
directly and need no alias. `runGit` now `mkdirSync`es each alias's parent and computes a relative
link target so a nested alias resolves. **`AGENTS.md` itself was not edited.**

## 7. Tests — real counts, all run

| suite | command | result |
|---|---|---|
| plugin-registry launch laws | `bun node_modules/.bin/vitest run --config ./🧪️tests/🎚️config/🟦️.ts 🧪️tests/🚀️launch` (from the registry bundle) | **10 passed / 10**, 43.1 s — capture `🗑️generated/z1-launch-tests.txt` |
| dev-target + zero-touch assertions | `bun 🐍️z1-dev-target-assertions.ts` | **6 assertions ok**, capture `🗑️generated/z1-dev-target-assertions.txt` |
| `workspace:setup` graph | `NX_DAEMON=false bun nx run workspace:setup --graph=stdout` | exit 0, 27 tasks, capture `🗑️generated/z1-setup-graph.txt` |
| launch.json freshness + identity | `bun 🐍️z1-launch-preview.ts` | 327 configs, 0 duplicates, render byte-identical to the committed file |
| launch target resolution | `bun 🐍️z1-launch-target-audit.ts` | 118 pairs, 0 unresolved — capture `🗑️generated/z1-launch-target-audit.txt` |

Tests added:
- `…/📇️registry/🧪️tests/🚀️launch/🟦️.ts` — new `describe("launch configuration identity")` with three
  laws: unique configuration names + `devLaunchNameSuffix` keeps the `👤️N` slot; every compound member
  resolves; every canonical root lifecycle command has a row. Discovery is memoized per file
  (`launchOutput()`), since a playground walk over 434 projects costs ~9 s and five tests need it.
- `…/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:1213+` — four renderer×variant assertions on
  `resolveNxInvocation` (including the bare alias honouring `SEMIO_PLUGIN`), plus two asserting the
  `@semio-tech/framework-os-dev:dev` target is renderer-neutral.
- `…/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts:14-20` + its fixture — the
  container-creation law rewritten from "post-create installs JS deps and has no `dependsOn`" to the
  zero-touch contract: `postCreateCommand` is `workspace:setup`, its `dependsOn` equals the fixture's
  13-entry `postCreateDependsOn`, every root-scoped entry exists, `deps-wasm` pulls `deps-trunk`, and
  the devcontainer Rust feature is pinned to `"none"`.

Test harness defect fixed on the way: `…/📇️registry/🧪️tests/🎚️config/🟦️.ts` had no `testTimeout`, so
vitest's 5 s default aborted **every** test in that bundle that walks the workspace — 2 were already
failing that way before I touched anything (capture shows `Test timed out in 5000ms`). Set to
`120_000`; with that plus the memoization the file is 10/10 green.

## 8. Honest gaps

- **Fresh-clone zero-touch is proven as a task graph, not as an execution.** I did not run
  `bun nx run workspace:setup` — it would fetch cargo/go/dotnet/playwright and build two Rust binaries
  on a machine already saturated by the cargo fleet. Idempotence and cross-platform safety of each leaf
  are read from source (§4), not observed. Only a real fresh clone / fresh container proves the whole
  chain.
- **Windows and Linux are unverified.** Every change is either platform-guarded source or JSON; I ran
  everything on macOS only. The `"version": "none"` Rust feature in particular changes image build
  behaviour and can only be confirmed by a container rebuild.
- **`repo:cache` (the `⚡️cache-contracts` suite) had not finished** at write time: 20 `PASS` lines
  including the container-bootstrap law I changed (`z1-cache-contracts.txt:12`), still executing the
  native Trunk cold-compile fixture (unrelated to this slice, minutes-long under fleet load). My new
  `resolveNxInvocation` assertions sit later in that file, so they are proven by the standalone
  `🐍️z1-dev-target-assertions.ts` run rather than by the suite.
- **No dev server was started and no browser was driven.** That `dev-s-react-dev` serves React is
  established by target resolution plus `ServeScript`'s `renderer !== "react"` guard, not by a live
  boot — C1b/B-slices own that.
- **A peer is refactoring `🚀️launch/🟦️.ts` live** (moving `command`/`env`/`serverReadyAction` from the
  seed into the generator). My seed-side P0 fix is absorbed by theirs; my name-prefix and
  `resolveNxInvocation` fixes are independent of it and were re-verified after their change landed.
- The 9 pre-existing HEAD name collisions are now merely *avoided* (the loser keeps a stale-emoji
  name) rather than properly named — e.g. `🛠️dev📸️shooting🎛️base⚛️react` keeps `📸️` instead of `🎥️`.
  Naming fixture rows correctly needs the seed to carry the discriminator explicitly; out of scope here.
- `go.work:1` pins `go 1.25` while the devcontainer feature installs `1.26` (G4 P2 #7). Harmless —
  Go's toolchain directive treats it as satisfied. Not changed.
- `♻️mit-bestand/🔎️recherche`'s 575 non-`.ts` scripts (G4 P2 #9) were not touched; it is a
  self-contained sibling workspace with its own `AGENTS.md`.

## 9. Files changed

| file | change |
|---|---|
| `.devcontainer/devcontainer.json` | `postCreateCommand` → `workspace:setup`; Rust feature → `"version": "none"` |
| `📋️project.json` | `setup.dependsOn` += `setup-git`, `prepare`, `repo-mcp:build`, `@semio-tech/framework-os-mcp-rs:build` |
| `📜️script.ts` | `AGENT_INSTRUCTION_ALIASES` export; `runGit` writes nested aliases with relative link targets; renderer-selection docstring corrected |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` | bare `:dev` alias honours `SEMIO_PLUGIN` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` | `frameworkOsPlaygroundDevEnv` docstring corrected |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | `dev` target de-hardcoded (no wgpu script, no `s`, no wgpu activation `dependsOn`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts` | `👤️N` kept in the suffix; rename skipped on name collision |
| `.vscode/🧩️launch.seed.jsonc` | `draw`/`s` canonical commands; 8 lifecycle rows; `served` row; dead `repo-cli-rs:mcp` row removed; `repo-coordinator-go:run` → `:dev`; bericht row renamed; 3 compound member lists fixed |
| `.vscode/launch.json` | regenerated (`@semio-tech/plugin-registry:generate`) |
| `README.md` | devcontainer step 4, toolchain line, four-golden-paths table, `setup git`/alias paragraph, canonical-commands paragraph |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts` | +3 laws, memoized render |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts` | `testTimeout: 120_000` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | +6 renderer/target assertions |
| `…/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts` + `🧫️fixtures/🚀️runtime-bootstrap/🔣️.json` | container-creation law rewritten to the zero-touch contract |

Ticket scratch (kept): `🐍️z1-nx-invocation-probe.ts`, `🐍️z1-launch-preview.ts`,
`🐍️z1-launch-target-audit.ts`, `🐍️z1-dev-target-assertions.ts`.
Captures (delete with the ticket): `🗑️generated/z1-*`.
