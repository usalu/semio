# G4 — Zero-touch setup + run-path audit

Read-only audit, slice G4 (Sonnet). Scope: `.devcontainer/**`, `.vscode/launch.json` +
`.vscode/🧩️launch.seed.jsonc`, root `📜️script.ts`/`📋️project.json`, the four golden-path command
chains, hygiene counts, README vs. reality. No files edited except this report. No cargo/nx builds or
servers were run; `bun nx show ...` was deliberately avoided (the task forbids nx builds and this session
had no cached `🗑️generated` captures to fall back on — `🗑️generated` was empty at start, per
`📓️status.md`'s "Session 2 ended with no worker alive"). Every claim below is grounded in reading source
files directly (exact paths/lines cited); where a claim would require running nx/cargo to fully confirm, it
is flagged as unverified rather than asserted.

## 1. `.devcontainer/**` — what it installs, what's missing

**Files**: `Dockerfile`, `devcontainer.json`, `docker-compose.yml`, `post-start.sh`, `post-attach.sh`,
`gitkraken-launch.sh`, `.dockerignore`, `README.md`. **`post-create.sh` does not exist** — confirms G1's
finding independently (G1 §"Devcontainer zero-touch"). It never existed as a separate file; the
post-create step is the one-liner `postCreateCommand` array inline in `devcontainer.json`.

**Dockerfile installs**: apt packages (build-essential/cmake/ninja/playwright's headless Chromium
dependencies/graphviz/sqlite3/mold/openjdk-21/…), bun `1.3.14` and Node `24.15.0` (pinned + sha256-checked,
architecture-aware amd64/arm64), Neo4j 5 + APOC jars, `uv` (Python). **`devcontainer.json` features** add:
Go `1.26`, Python `3.14`, .NET `8.0`/`9.0`/`10.0`, **Rust `1.92`** with `targets: "wasm32-unknown-unknown"`
only, git/git-lfs/gh, sqlite.

**Finding — Rust toolchain mismatch (P1).** `rust-toolchain.toml:1-4` pins
`channel = "nightly-2026-07-07"` with `targets = ["wasm32-unknown-unknown", "wasm32-wasip2"]`. The
devcontainer feature installs a *different*, stable `1.92` toolchain with only one of the two targets.
Because `rustup` auto-installs/-overrides the pinned nightly the first time `cargo`/`rustc` runs inside the
repo (the `1.92` toolchain is never actually used), this doesn't break correctness, but it means: (a) the
`1.92` feature install is wasted image weight, (b) `wasm32-wasip2` (needed by every plugin's Rust component)
is added only lazily on first `deps-wasm` run against the *right* toolchain, not at image-build time, and
(c) README's "The devcontainer includes: … Rust 1.92" (line 525) is simply wrong about which toolchain
actually compiles anything.

**Finding — `postCreateCommand` only installs JS deps (P0).**
`devcontainer.json:37`: `"postCreateCommand": ["bun", "nx", "run", "workspace:deps-javascript"]`. That nx
target (`📋️project.json:deps-javascript`) runs only
`🧰️framework/…/📦️dependencies/📜️script.ts sync` (a `bun install`-equivalent). The root `setup` nx target
(`📋️project.json:setup`) — the one that actually fetches everything — `dependsOn: [deps-javascript,
deps-python, deps-cargo, deps-go, deps-dotnet, deps-cpp, deps-browsers, deps-wasm, deps-tools]` — is **never
invoked** by the devcontainer. Concretely, a fresh devcontainer boot never runs:
- `cargo fetch --locked` (`deps-cargo`) — first `cargo build` anywhere pays the full fetch.
- `go mod download` (`deps-go`) — first Go MCP build/test pays it.
- `rustup target add wasm32-wasip2` + pinned `wasm-pack`/`wasm-bindgen-cli` install (`deps-wasm`,
  `🧰️framework/…/📦️dependencies/🏗️native/📜️script.ts:36-41`) — every plugin wasm build needs this.
- `playwright install chromium` (`deps-browsers`, same file line 34) — every headless probe needs this
  (Playwright's own apt deps ARE baked into the Dockerfile image, but the browser binary itself is not).
- `cargo install cargo-nextest`/`cargo-llvm-cov` (`deps-tools`).

  A fresh devcontainer clone thus fails or silently free-rides on whatever happens to already be cached the
  moment anyone runs `dev s`, `dev mcp`, a plugin's `cargo test`, or a Playwright probe — none of which is
  "zero-touch" as README's step 4 ("Wait for container build and setup to complete") implies.

**Finding — `setup git` / root-alias symlinks never run automatically (P1).** `📜️script.ts:354-368`
(`SetupScript.runGit`) is what creates the `CLAUDE.md`/`GEMINI.md` symlinks to `AGENTS.md` (README line
675-677 describes this) and removes legacy git hooks. It is a **separate CLI segment**
(`bun ./📜️script.ts setup git`) — it is **not** part of the `setup` nx target's `dependsOn`, not part of
`postCreateCommand`, and not part of `prepare`. Verified live on this very checkout: `CLAUDE.md` and
`GEMINI.md` do not exist at the repo root at all (`ls` confirms no file, no symlink), even though `AGENTS.md`
does — i.e. this exact gap is not hypothetical, it is the checkout's current state. The task description's
"three symlinks at the repo root" does not match what's on disk today: only two root symlinks exist
(`.generation3d-edit-link`, `.generation3d-crate-link`, both unrelated scratch links into
`✏️s/🔌️plugins/🌀️procedural/…`, tracked in git), and zero of the `setup git`-produced aliases exist. Whatever
prompted "three" is stale relative to this tree.

**Finding — `prepare` (schema/tokens/assets/graph/plugin-registry generation) is also never auto-run.**
`📋️project.json:prepare` depends on `@semio-tech/framework-schema:generate`,
`@semio-tech/ui-styling-tokens:generate`, `@semio-tech/assets:build`, `@semio-tech/framework-graph:generate`,
`@semio-tech/plugin-registry:generate`. The plugin registry catalog it produces
(`🧰️framework/…/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`, 1476 lines per
`📓️audit-os-frontend.md` §2) is **gitignored** (`.gitignore:91: **/🤖️generated/`) and not part of `setup`'s
`dependsOn` either. A fresh clone + full `bun nx run setup` still has no plugin registry, no generated
schema, no built assets — `dev s`/`dev <plugin>` cannot resolve a playground without it.

**Finding — MCP binaries need a manual pre-build (P1, blocks outcome d out of the box).**
`.mcp.json` launches `bun ./📜️script.ts dev mcp stdio client` (repo) and
`… dev mcp stdio os --scopes …` (semio). Both gate on a pre-built binary and throw a clear, actionable error
if missing rather than building it inline (by design, per the comment at `📜️script.ts:231-232`: "a
continuous `dev` session must stay thin"): `requireRepoMcpBinary` (`📜️script.ts:233-237`, message
`"repo MCP client binary is missing at …; run: bun nx run repo-mcp:build"`) and `requireMcpBinary`
(`🧰️framework/…/🌉️mcp/🟦️.ts:44-54`, message `"semio-os-mcp binary gate failed at …"`). Neither
`repo-mcp:build` nor the os-mcp build target is in `setup`'s `dependsOn`. Practically: on a fresh
clone/devcontainer, the moment an AI agent (this very kind of Claude Code session) tries to use the `repo`
or `semio` MCP server via `.mcp.json`, it fails until a human separately runs the two build targets — the
outcome the ticket calls "AI integration over MCP" is not zero-touch today even after `bun nx run setup`.

**CLT/Xcode fallback (verified present, cross-platform-safe)**: `ensureAppleDeveloperDir()` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/📤️publication/🟦️.ts:3-12` prefers
`/Library/Developer/CommandLineTools` over an unlicensed `Xcode.app` (`cargo`/`cc` otherwise exit 69),
guarded by `process.platform !== "darwin"` so it's a correct no-op elsewhere. This is the fix referenced by
project memory `project-xcode-license-breaks-native-link-use-clt.md` and B3a's gis fallback — it exists and
is wired at the right layer.

**wasm-tools/jco**: `jco` is `@bytecodealliance/jco`, an ordinary npm dependency resolved via
`resolveWorkspaceBin` (`🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:66-68`) — covered by
`deps-javascript`/`bun install`, no separate native install needed. A literal `wasm-tools` CLI binary is
**not** installed or invoked anywhere in the codebase (the one hit, `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:957`,
is only an error-message string) — not a gap, just worth noting the checklist item resolves to "nothing to
install here."

**Go/`.NET` versions**: `go.work:1` pins `go 1.25`; the devcontainer feature installs `1.26` (compatible,
Go's toolchain directive treats this as satisfying the `go 1.25` floor — not a real break, just an
unpinned-exact-match note).

## 2. Golden paths — command chains

### (a) `dev s` — os React/wgpu frontend

Two independent entry points exist and they are **not equivalent**:

1. **Root CLI**: `bun ./📜️script.ts dev s` → `DevScript.run` (`📜️script.ts:408-436`) branch
   `segments[0] === "s"` → `runFrameworkOsPlaygroundDev("s", …)` (`📜️script.ts:272-279`) →
   `runCmd("bun", ["nx","run","@semio-tech/framework-os-dev:dev","--","s",…rest], {env: frameworkOsPlaygroundDevEnv(...)})`.
2. **`.vscode/launch.json`** (what a human clicks): row `"🛠️dev🪐️space⚛️react"`
   (`.vscode/launch.json:3056-3075`, generated from seed `devLaunchers["s"]` at
   `.vscode/🧩️launch.seed.jsonc:3151-3182`) — command is the **literal string**
   `"bun nx run @semio-tech/framework-os-dev:dev"` with `env: {S_OS_PORT:"6070", SEMIO_PLUGIN:"s",
   SEMIO_RENDERER:"react"}`, no extra CLI args.

Both paths hit the same nx target: `@semio-tech/framework-os-dev`'s `dev` target
(`🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:"dev"`), whose `command` is fixed,
non-templated text:
```
bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve s dev
```
That script's `ServeScript.run` (`🧰️framework/…/🧊️wgpu/🌐️server/📜️script.ts:9-25`) **unconditionally**
does `process.env.SEMIO_RENDERER = "wgpu";` at line 21, then loads
`🧊️wgpu/🌐️server/🎚️config/🟦️.ts`, which imports only `createWgpuBrowserConfig` — there is no react branch in
that config at all.

**Finding — the "⚛️react" `dev s` launch rows can never serve React (P0).** Every `.vscode/launch.json` row
that invokes `@semio-tech/framework-os-dev:dev` — `🛠️dev🪐️space⚛️react` (port 6070),
`🛠️dev🖥️s👤️1⚛️react` (port 6072), `🛠️dev🖥️s👤️2⚛️react` (port 6073) — sets `SEMIO_RENDERER: "react"` in its
`env` block, but that value is dead: the target's command is invariant and forces `SEMIO_RENDERER=wgpu`
before doing anything else. Clicking any of these three rows serves the **wgpu/wasm** build on the labeled
port, not React. The sibling `🧊️wgpu` rows (`🛠️dev🪐️space🧊️wgpu🌐️wasm`, `👤️1🧊️wgpu🌐️wasm`, `👤️2🧊️wgpu🌐️wasm`)
call the *identical* command — they differ from the "react" rows only by port number and the (equally dead)
`SEMIO_RENDERER` value. This is directly triangulated by how the ticket's own collaboration E2E harness gets
a real React server: `🧰️framework/…/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts:281-290` calls
`spawnDaemon("bun", [devScript, "serve", "s", "react", "dev"], …)` where `devScript` is
**`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`'s own `serve` verb**
(`ServeScript` at `🧰️framework/…/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:45-56`, which rejects anything but
`renderer === "react"` and boots the real Vite config at `../../🏗️builder/🌐️vite/🟦️.ts`). That command has
**no `.vscode/launch.json` row at all** — the only way a human currently reaches the working React "s"
server through the IDE's Run panel is to not use any of the "s" rows and instead hand-type
`bun ./📜️script.ts serve s react dev` in a terminal, which directly contradicts AGENTS.md's "All devs are
using `launch.json` and never use the cli" rule (line 50).

Net effect on outcome (c): the two-user collaboration launch rows a developer would reach for
(`🛠️dev🖥️s👤️1⚛️react`/`👤️2⚛️react`) do not exercise the React UI the collaboration E2E scenario itself
uses and was written against — they silently open two wgpu windows instead.

Root cause is a single generator input:
`.vscode/🧩️launch.seed.jsonc:3154` (`devLaunchers["s"].command`) is the one field the generator
(`🚀️launch/🟦️.ts:92-107`) reuses for both `react` and `wgpu` renders of a variant; every *other* playground
variant's `command` is the renderer-agnostic `bun nx run workspace:dev -- <variant>` (root `DevScript`,
which really does branch on env), but "s" is special-cased to the wgpu-only nx target because of its
`activate-s-wgpu-dev` dependency chain (comment at `📜️script.ts:253-271` explains the historical intent —
"served forces react … because `frameworkOsPlaygroundDevEnv` defaults SEMIO_RENDERER to wgpu" — but that
logic lives in the *root* `dev` verb, not in the `@semio-tech/framework-os-dev:dev` nx target the seed
actually points the launch rows at).

**Separately, confirmed still-open from `📓️audit-os-frontend.md`**: no host-side Rust consumer of the
`activationEvents`/`on-artifact-kind:*` registry field was found (grepped again this session, same result),
so "boots all plugins" remains unconfirmed at the runtime-activation level independent of the react/wgpu bug
above.

### (b) `os-hub:dev` — hub backend

`.vscode/launch.json:"🛠️dev🗄️os-hub"` → `bun nx run os-hub:dev`, `env: {OS_HUB_PORT:"8787",
OS_HUB_DATA:"${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"}` → `🌎️hub/📦️packages/🦀️rust/📋️project.json:"dev"`
(`dependsOn: [build-dev, {target:"build", projects:["os-hub-admin"]}]`) → `bun ./📜️script.ts dev` (the hub
bundle's own script.ts, compliant) → `DevScript` at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:12034` onward. This
chain is well-behaved: cross-platform `.exe` suffixing, `process.platform` guards throughout
(`chmodSync`/`symlinkSync` skipped on `win32`, junctions used instead of symlinks on Windows at lines
9067/9103), spawns with `shell:false`. No macOS-only commands found in this file. `OS_HUB_PORT`/`OS_HUB_DATA`
are documented env vars with sane defaults (`resolve(process.env.OS_HUB_DATA ?? join(repoRoot, ".🧬semio",
"🌐hub"))`, line 12054).

### (c) Two-user collaboration scenario

Two distinct mechanisms exist, and only one is currently reachable from the IDE:
- **Automated**: `.vscode/launch.json:"🛠️dev🤝️os-collab-e2e"` → `bun nx run
  @semio-tech/framework-os-dev:collab-e2e` → that bundle's own `📜️script.ts verify collab` →
  `runCollabE2eVerify` (`🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts:799`), which correctly spawns the hub
  (`hubScript … dev`) and **both** user dev servers via the real react `serve` verb (line 283, see above) on
  scanned free ports (`collabScanPort`, `isDevPortInUse("127.0.0.1", port)` — cross-platform via Node's
  `net` module, no `lsof`/`netstat` shelling). This path is solid and OS-agnostic.
- **Manual, IDE-driven**: the `👤️1`/`👤️2` launch rows (see §(a)) — broken as described, always wgpu.
  `S_HUB_URL`/`S_DATA_DIR` env values in these rows (`.🧬semio/🔗space/s-user{N}`) are otherwise sane and
  match the automated harness's data-isolation convention, so fixing the command (not the env) is the whole
  fix.

Per `📓️g1-goal-gap-audit.md` §1 Outcome 3, the 10-step `collabRunScenario` itself has not been run end to end
in this ticket regardless of the frontend bug above — this audit did not attempt to run it (no
servers/builds allowed for this slice).

### (d) `dev mcp stdio os` / repo MCP

`.mcp.json` (`repo`/`semio` entries) and the equivalent launch rows both resolve through
`📜️script.ts:DevScript.runMcp` (`📜️script.ts:531-566`):
- `dev mcp stdio os …` → `runMcpOs("stdio", extra)` (line 592-598) → `runCmd(requireMcpBinary(this.root),
  args, …)`.
- `dev mcp stdio` (no `os`) / `dev mcp stdio client` → `runMcpStdioRepo(slugs)` (line 600-608) →
  `runCmd(requireRepoMcpBinary(this.root), [], {env: {GOWORK: …, SEMIO_REPO_MCP_CLIENT: profile}})`.

Both binary resolvers are cross-platform (`.exe` suffix checks, `X_OK` access check skipped on `win32`) and
fail with an actionable message rather than a stack trace when the binary is missing — good design — but as
noted in §1, nothing in `setup` actually builds those binaries, so the "good error message" is the steady
state on a fresh clone, not a fallback for an edge case. `.mcp.json` itself has no hard-coded absolute paths
(uses `./📜️script.ts`, relies on cwd = repo root, which is how Claude Code launches configured MCP servers).

## 3. Hygiene — `script.ts`-only discipline, stray scripts, launch/target drift

**`📋️project.json` calling something other than `📜️script.ts`**: **0 violations** found among 436 real
project.json files (excludes `🎫️tickets/**`, `dist/**`, `node_modules/**`, and `temp/**` — the last is a
gitignored, 0-files-tracked scratch mirror of an unrelated old `compose` tree that legitimately doesn't
follow repo conventions; scanning it produced 24 false "offenders" that are out of scope). Every real
target's `command`/`commands` either invokes its own local `📜️script.ts` or `nx run <other-project>:<target>`
(which itself resolves to that project's `📜️script.ts`). This rule is genuinely well-enforced.

**Script files other than `📜️script.ts` outside ticket folders**: after excluding taxonomy fixture-kind
files (hundreds of `🐍️.py`/`🦀️.rs`/etc. single-glyph-named files under `🧪️tests/…/mutate-*` — these are test
*payload* content, not orchestration scripts) and the separate `♻️mit-bestand/🔎️recherche` research tree
(575 hits — has its own `AGENTS.md`/`.cursor`/`.claude`, clearly a self-contained sibling workspace, not
audited further here), the only real non-`.ts` executable files in the primary Nx workspace are:
- `.devcontainer/{post-start,post-attach,gitkraken-launch}.sh` — devcontainer lifecycle hooks; devcontainer
  itself only knows how to run shell commands, so this is a structurally necessary exception.
- `🧰️framework/…/🔩️native/🥾️bootstrap/🐚️.sh` (+ a sibling `🔵️.ps1` for Windows) — the native
  (non-container) bootstrap entry invoked by `NativeOsScript` (`📜️script.ts:283-308`); a legitimate
  chicken-and-egg exception since bun/node don't exist yet when this runs.
- Two `🟨️.mjs` Nx-plugin files (`🧰️framework/…/📚️library/🟨️.mjs`, `…/🧪️test/🟨️.mjs`) registered directly in
  `nx.json:50,55` — Nx loads plugin files itself and cannot transpile TS for its own plugin-loading step, so
  `.mjs` is required here, not a violation.
- One `__init__.py` (a real Python package file, not a script).

  **Net: zero rule violations** — every non-`.ts` executable in the workspace is one of these four justified
  categories.

**Launch rows pointing at missing targets**: checked all 212 distinct `bun nx run …` command strings in
`.vscode/launch.json`; **all 28 distinct target-projects they reference exist** (verified by cross-referencing
every referenced project name against the `name` field of all 434 discovered `📋️project.json` files — zero
misses). Full target-level (not just project-level) validation of the ~180 project-scoped target names was
not exhaustively done: most belong to `@semio-tech/framework-os-dev`/`@semio-tech/framework-renderer-wgpu`
and are generated **dynamically** by the Nx plugin at
`🧰️framework/…/📚️library/🟨️.mjs:1004-1035` (`playgroundPreparationTargets`, one `dev-<variant>-{react,wgpu}-<profile>`
per registered playground) rather than declared statically in `project.json` — confirming each one exists
would require `bun nx show project <x>`, which this slice deliberately did not run (no-nx-build
constraint). **One previously-flagged gap is now fixed**: G1 #9's missing
`⚖️gate🧱️hub-foundations📐️source` row exists in both the seed (`.vscode/🧩️launch.seed.jsonc:2268`) and
generated `launch.json:4036`, targeting `os-hub:foundation-source-check`
(`🌎️hub/📦️packages/🦀️rust/📋️project.json:541`), which exists — resolved by a peer since that audit.

**nx targets with no launch row (P1, structural, large)**: the root `📋️project.json` declares 291 targets.
Grepping the seed for exact `workspace:<target>` references shows **zero** `.vscode/launch.json` rows for
the nine other top-level lifecycle commands README calls "canonical root commands" controlling "All
automation, CI runs, and agent workflows" (README line 714): `test`, `build`, `lint`, `format`, `publish`,
`purge`, `generate`, `start` — only `setup` has a row (order shown further up the seed under `4_build`/gate
groups, one exact hit for `workspace:setup`). None of `bun nx run workspace:test`, `…:build`, `…:lint`,
`…:format`, `…:publish`, `…:purge`, `…:generate`, `…:start` appears anywhere in
`.vscode/🧩️launch.seed.jsonc`, confirmed by direct string search (`grep -c "workspace:test"` → 0, same for
`build`). This directly contradicts AGENTS.md line 50: *"All devs are using `launch.json` and never use the
cli. You MUST register all executable commands there."* Today, the most fundamental commands in the repo can
only be run from a terminal (`bun run test` via root `package.json`), not from VS Code's Run panel. Beyond
these nine, roughly 250 of the remaining 291 root targets are granular per-law `test`/`verify-policy-breach`/
`*-document-contract`/`*-window-ownership` micro-targets that are plausibly meant to be exercised only via
the aggregate `test`/`verify` commands rather than individually — but since those aggregates themselves have
no row either, there is currently no IDE-launchable path to any of them.

## 4. README "getting started" vs. reality

- Line 521: *"Wait for container build and setup to complete"* — reality: `postCreateCommand` only installs
  JS deps (§1). No cargo/go/wasm/browser/tool installation, no plugin-registry/schema generation, and no MCP
  binary build happens during container creation; "setup" in the README's sense never actually completes
  automatically.
- Line 525: *"Rust 1.92"* — reality: the pinned, actually-used toolchain is nightly-2026-07-07
  (`rust-toolchain.toml`); 1.92 is a stable toolchain the devcontainer feature installs but rustup overrides
  on first use inside the repo.
- Lines 675-677: root aliases (`CLAUDE.md`, `GEMINI.md`) "are recreated … when you run `bun
  ./📜️script.ts setup git` (also invoked from `npm run setup`)" — this is **false** as written: `npm run
  setup`/`bun nx run workspace:setup` does **not** invoke `setup git` (verified: root `setup` target's
  `dependsOn` is the nine `deps-*` targets only, and the bare `setup` CLI verb — `SetupScript.runFull`,
  `📜️script.ts:370-372` — just logs a message). This checkout is live proof: `CLAUDE.md`/`GEMINI.md` do not
  exist here.
- Lines 530-547 (Windows/native setup) and 557-561 (macOS/Linux native) are consistent with what
  `NativeOsScript`/`NativeDependenciesScript` actually implement — this section reads accurately.
- Line 714 ("canonical root commands… `setup`, `start`, `dev`, `generate`, `lint`, `format`, `test`,
  `build`, `publish`, and `purge`") is accurate as a description of `package.json`, but combined with
  AGENTS.md's "never use the cli" rule and §3's finding, the README and AGENTS.md describe two different,
  currently-incompatible workflows for the same nine commands.

## Ranked fix list (worker-sized)

**P0**
1. **Fix the "s" React launch rows to actually serve React.** File: `.vscode/🧩️launch.seed.jsonc:3151-3182`
   (`devLaunchers["s"]`). Give `reactEnv`'s renderer a *different* `command` than `wgpuEnv`'s — point react at
   `bun ./📜️script.ts serve s react dev` (via `@semio-tech/framework-os-dev`'s own `serve` verb,
   `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:45`) or add a matching `S_HUB_URL`/`S_DATA_DIR`-aware nx target for
   it, keeping `wgpuEnv`'s command as the existing `@semio-tech/framework-os-dev:dev`. Regenerate
   `.vscode/launch.json`. This is the single highest-leverage fix for both outcome (a) and the two-user
   collaboration scenario (outcome c), and the generator field (`DevLauncherEntry.command`) already supports
   per-renderer differences elsewhere — "s" just isn't using that capability. Add a users/react smoke test
   asserting the served bundle is Vite/React, not `trunk`/wgpu, so this can't silently regress again.
2. **Wire `bun nx run setup` to actually reach zero-touch.** File: `📋️project.json:"setup"`. Either fold
   `setup git`/`setup prepare`/MCP binary builds into `setup`'s `dependsOn`, or change
   `.devcontainer/devcontainer.json:37`'s `postCreateCommand` to `["bun","nx","run","setup"]` **and** append
   `setup-git`/`prepare`/`repo-mcp:build`/the os-mcp build target. Update README's step 4 and the "Rust 1.92"
   claim (line 525 — say "nightly, pinned by `rust-toolchain.toml`") to match.

**P1**
3. **Register the 8 missing top-level lifecycle commands in `.vscode/launch.json`.** File:
   `.vscode/🧩️launch.seed.jsonc` — add `4_build`/gate-group rows for `test`, `build`, `lint`, `format`,
   `publish`, `purge`, `generate`, `start` (mirroring the existing `⚖️gate🧱️hub-foundations📐️source`
   pattern), closing the gap against AGENTS.md line 50.
4. **`setup git` (CLAUDE.md/GEMINI.md symlinks + legacy hook removal) never runs automatically anywhere.**
   File: `📜️script.ts:354-368` + `📋️project.json:"setup"`. Fold it into the `setup` nx target's `dependsOn`
   (it's fast and idempotent) so AI-agent instruction files exist after a fresh clone without a manual step.
5. **MCP binaries require an undocumented manual build.** Files: `📜️script.ts:233-237`,
   `🧰️framework/…/🌉️mcp/🟦️.ts:44-54`. Either add `repo-mcp:build` + the os-mcp build target to `setup`'s
   `dependsOn`, or document the manual step prominently in README's MCP section so a fresh Claude Code
   session doesn't fail cold on `.mcp.json`.
6. **Rust toolchain feature mismatch.** File: `.devcontainer/devcontainer.json:25-28`. Either pin the
   `rust` feature to the same nightly channel as `rust-toolchain.toml` (avoiding a redundant second
   toolchain download) or add `wasm32-wasip2` to the feature's `targets` so the image-baked toolchain is at
   least self-consistent even before the override kicks in.

**P2**
7. `go.work:1` (`go 1.25`) vs. devcontainer's Go feature (`1.26`) — harmless today, worth pinning exactly for
   reproducibility.
8. Audit-scale gap, not a single fix: ~250 granular per-law `test-*`/`verify-policy-breach-*`/
   `*-document-contract` root targets have no individual launch row and are only reachable if/when the
   aggregate `test`/`verify` rows from item 3 are added — no action needed beyond item 3 unless a specific
   micro-target needs standalone debugging.
9. `♻️mit-bestand/🔎️recherche` is a self-contained sibling research tree with its own tooling conventions —
   confirm with its owner whether it's meant to be exempt from AGENTS.md's `script.ts`-only rule (it appears
   to be, structurally, but this audit did not confirm that explicitly).

## Honest gaps

- Did not run `bun nx show project <x>` for any of the ~180 dynamically-generated per-playground targets, so
  "launch rows pointing at missing targets" is fully verified only at the project level, not the target
  level, for those targets (no-nx-build constraint for this slice).
- Did not attempt to run the collaboration scenario, `dev s`, or any MCP handshake — this is a static/code-
  reading audit only, consistent with the slice's read-only, no-builds/no-servers mandate.
- `♻️mit-bestand/🔎️recherche`'s 575 `.py`/`.sh`/`.mjs` files were counted but not individually inspected.
