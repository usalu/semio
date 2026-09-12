# Build / restage / gates audit — 2026-09-12 (read-only)

No source edited, no cargo/nx build started, no git state-modifying command run. `repo`/`semio` MCP
servers failed to connect this session (`invalid initialize params` / `CONNECTION_CLOSED`) — everything
below comes from reading source and disk directly, not from repo MCP resources.

## Headline

The served wasm on `http://127.0.0.1:6018/?plugin=generation3d` is currently **stale**: it was staged
2026-09-11 **15:51:26** CEST (94 344 543 B core wasm, 220 927 B descriptor — a *later* restage than the
14:15 one `📓️unknown-kind-after-restage-2026-09-11.md` describes), but the procedural source tree kept
moving after that and a 73-file, 4475+/591- diff (generation3d editor commands, tick-addressing fixture,
`plugin.rs` +1027, `ShellHost`/`World3dHost`/`PluginRuntime`/`Interpreter`/`Tree`) landed with mtimes up to
**15:59–16:09** and has since been **auto-committed** as `989582baab` (2026-09-12 01:07:58 CEST, verified
`git show --stat`). **A restage is needed** to serve that commit's procedural changes. `git status` is
clean under `✏️s/🔌️plugins/🌀️procedural` as of this audit (01:09 CEST) — nothing left staged for this
lane; the auto-commit already did what §6 anticipated.

## 1. Restage recipe

**Crate**: `semio-s-plugin-procedural` (the whole plugin — one wasm component bundles `generation3d`,
`generation2d` and the `🧩️assembly` artifact; there is no separate `semio-s-artifact-procedural-generation3d`
crate). Confirmed at `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts:describePluginComponent(...,
"semio-s-plugin-procedural", ...)` and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:367`
(`buildPluginComponent`).

**Cargo command** (from `buildPluginComponent`, `🔌️plugin/🖨️describe/…/📜️script.ts:368`):
```
cargo build -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev
```
`component-app-assembly` is a **feature of the `🧩️assembly` sub-crate**
(`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/Cargo.toml:19`), not a flag passed to
this build — the top-level plugin crate does not gate on it.

**Env** — root `Cargo.toml:474-476` `[profile.wasm-dev]` is `inherits = "dev"` + `codegen-units = 1` with
**no `debug = false`**, so `CARGO_PROFILE_WASM_DEV_DEBUG=false` is NOT a repo default; it must still be
exported by the caller (per `[[project-wasm-dev-profile-debug-off-and-swap-thrash]]`) or the rustc child can
balloon to multi-GB RSS under swap. `devToolingEnv()` (the env every build/describe call uses,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts:10`) only strips
`NODE_OPTIONS`/`VSCODE_INSPECTOR_OPTIONS` and pins a few `NX_*` flags — it passes through whatever
`CARGO_PROFILE_WASM_DEV_DEBUG`/`SEMIO_RENDERER`/`NX_DAEMON` the caller already has in `process.env`.
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📋️project.json` declares no `component-dev`/`describe`
target env directly — env comes from the shell that invokes nx/bun. The ticket's own
`📜️serve-generation3d-react.sh` exports `NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0` for the
**serve** step only; no restage script in this ticket exports `CARGO_PROFILE_WASM_DEV_DEBUG`, so set it
explicitly before restaging.

**Target dir / lock** — since 2026-09-11 (`[[project-shared-cargo-build-dir-fine-grain-locking]]`) every
cargo run shares ONE build-dir: root `.cargo/config.toml` sets `target-dir =
.🧬semio/🦑️repo/⚡️cache/cargo/target`, `build-dir = .🧬semio/🦑️repo/⚡️cache/cargo/build`,
`[unstable] fine-grain-locking = true`. **Never set a private `CARGO_TARGET_DIR`.** Checked right now: no
`.cargo-lock` under `…/cargo/target/wasm32-wasip2/` (no wasm build in flight); two unrelated native
`cargo` pids (16829, 17456) hold `…/cargo/target/debug/.cargo-lock` — fine-grain locking means that does
**not** block a `wasm32-wasip2`/`wasm-dev` build, which takes its own per-unit locks.

**Where restage output lands** — two DIFFERENT `🔌️plugin-modules/` trees exist; only one is served:

| tree | who writes it | who reads it | current `🌀️procedural` state |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/` | `activatePlaygroundRuntime`/`PreparationScript` (component-dev → jco transpile → stage) | **`⚙️vite.config.ts`'s `pluginModulesDir`** (`🔌️vite-plugins.ts:33`, `path.resolve(configDir, "../../../🔌️plugin/📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules")`) — this IS what vite serves on 6018 | `semio_s_plugin_procedural_component.core.wasm` **94 344 543 B**, `🛂️.descriptor.semio` **220 927 B**, `🔣️.json` **969 298 B**, all mtime **2026-09-11 15:51:26** — byte-identical to what `curl` returns from 6018 right now (verified) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/` (`PLUGIN_MODULES_ROOT`, `🔌️vite-plugins.ts:22`) | the wgpu native-runner's own `PreparationScript` path (`📜️script.ts:1331`) | wgpu native runner only, **not** the react dev vite serve | `semio_s_plugin_procedural_component.core.wasm` **82 246 182 B**, descriptor **200 859 B**, mtime **2026-09-10 23:44** — stale, but irrelevant to 6018 |

Do not mistake the second tree for the served one (this exact confusion produced a wrong byte comparison
mid-audit before the vite config was read).

**Exact restage commands**, cheapest→most complete:
```bash
# rebuild + re-stage + regenerate the activation receipt vite watches (does the cargo build)
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
# equivalently, from the dev TS package:
bun ./📜️script.ts dev generation3d          # activates (unless `served` is passed) THEN serves
```
`serve generation3d react dev` (what `📜️serve-generation3d-react.sh` runs) does **not** rebuild — see §2.

**Does the running vite pick up a new wasm without restart?** Yes. `ServeScript` (`📜️script.ts:1452`)
reads the activation receipt once at startup and hands the static `pluginModulesDir` to Vite; the receipt
is watched live by `semioActivationVitePlugin`, and the plugin-modules route is a plain static-file mount
(`🔌️vite-plugins.ts:168,176`) — so re-running just the `activate-…` step (or `dev generation3d` without
`served`) while `g3dreact`'s vite keeps running is sufficient; confirmed in
`📓️rebuild-2026-09-09.md` §6 ("a fresh vite always serves the current staging — there is no import-time
snapshot to invalidate") and consistent with what I found here (the currently-running vite, pid 3758 under
screen `g3dreact`, is already serving the 15:51 bytes without having been restarted since).

**wgpu variant**: `📜️serve-generation3d-viewer.sh` / screen `semio-g3d-viewer` is a **react** editor
window pinned to port 6019, not the wgpu renderer — despite the ticket brief's phrasing, `VITE_SEMIO_RENDERER=react`
is set explicitly in that script. The actual wgpu native-runner path is `🎮️dev🧩️generation3d⚛️react
release`'s sibling wgpu targets / `.claude/launch.json`'s `procedural3d-wgpu`
(`bun nx run @semio-tech/framework-os-dev:dev -- generation3d`, port 6118, `SEMIO_RENDERER=wgpu`) — separate
build path (`buildEngineWasm`/trunk), not covered by the react restage above. Not probed live this audit
(process check only): no wgpu trunk process found running on 6118 right now (`lsof -iTCP:6118` — no
listener); only 6018 (react editor, pid 3758) and 6019 (react viewer, pid 49348) are live.

## 2. Does `serve generation3d react dev` rebuild on source change?

**No — neither `serve` nor `dev` watches the filesystem for Rust changes.**

- `ServeScript.run` (`📜️script.ts:1452-1460`) only reads the existing activation receipt
  (`readActivationReceipt`) and starts `runViteBunxDev` — no cargo call anywhere in this method.
- `DevScript.run` (`📜️script.ts:1478-1489`), for the react branch: `if (!served) await
  activatePlaygroundRuntime(plugin, profile); await new ServeScript(this.root).run(...)`. This is **one
  activation at process start**, not a watcher — `activatePlaygroundRuntime` runs once, synchronously,
  before vite even boots. Passing the `served` segment (as `📜️serve-generation3d-react.sh` effectively
  does by calling `serve` directly, not `dev`) skips activation entirely.

So a Rust edit after the process is already serving is invisible until someone re-runs the `activate-…`
step (or `dev <variant>` without `served`) by hand. **This is exactly why every prior session in this
ticket restaged manually** (`📓️rebuild-2026-09-09.md`, `📓️unknown-kind-after-restage-2026-09-11.md`,
`📓️wgpu-boot-watchdog-2026-09-10.md` §8's `[watch]`-driven wgpu bundle is a *different*, browser-boot-only
mechanism unrelated to the plugin wasm) — there is no dev-mode HMR/watch path for the guest wasm in this
repo, by design (`SEMIO_VITE_HMR=0` is set in the serve script for the same reason: HMR only ever applied
to the JS/TS host shell, never to the guest component).

## 3. Native and TS twin test recipes

**Native (Rust)** — nx target `test` on `@semio-tech/procedural-plugin`
(`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📋️project.json`):
```
bun nx run @semio-tech/procedural-plugin:test          # fundamental level
bun nx run @semio-tech/procedural-plugin:test-quick
bun nx run @semio-tech/procedural-plugin:test-long
bun nx run @semio-tech/procedural-plugin:test-exhaustive
```
Router: `bun ./📜️script.ts test [quick|long|exhaustive]` → `runCargoTestBudgeted(["semio-s-plugin-procedural"],
...)`. No `--features` passed (so `component-app-assembly` is off unless the sub-crate's own Cargo.toml
defaults it on — not checked here, out of scope for a read-only pass). `RUST_MIN_STACK` defaults to
**134217728** (128 MiB) inside `runCargoTestBudgeted`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1662-1663`) unless the
caller already set one — this crate's `TestScript` does not override it. Cargo-nextest equivalent (what
actually runs when nextest is on `PATH`, `🟦️.ts:1702+`):
```
RUST_MIN_STACK=134217728 cargo nextest run -p semio-s-plugin-procedural --profile fundamental
```
(swap `fundamental` for `quick`/`long`/`exhaustive`; each level appends `--skip <higher-level>::` filters).

**TS twin** — `contract.ts` files are standalone third-party-oracle scripts, not wired into any vitest
suite or the `@semio-tech/procedural-js` package's own `test` target (its `📜️script.ts` only runs a fixed
list of `📚️examples/…/🧩️example/🟦️.ts` fixtures — `🔬️tick-addressing/contract.ts` is not in that list).
Confirmed via repo-wide grep: `🔬️tick-addressing` appears only in Nx's own file-map, never imported by
another `.ts`. Run it directly:
```
bun ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/contract.ts
```
It `JSON.parse`s the SAME fixture the Rust unit test decodes
(`🧫️fixtures/🪟️tick-addressing.json`, `format: "semio.generation3d.tick-addressing"`) and asserts the
`generatePreview` window-kind arming/dispatch rows with `node:assert/strict` — the "same output, third-party
parser" pairing CLAUDE.md's test-driven rule asks for. Both the fixture and this contract were touched in
the now-committed 73-file diff (`+25` on `contract.ts`, `+146/-…` on the fixture) — a restage is needed for
the *runtime* to reflect them, but neither test needs a restage to run (both read the fixture from source).

## 4. Gates

| gate | command | last known status for procedural |
|---|---|---|
| Dependency Truth Gate | `bun ./📜️script.ts verify dependencies literal-external` | **Red, repo-wide, pre-existing** — `target=0, current=193, oracle-conflicts=23` (`📓️test-harness-audit-2026-09-09.md`, byte-identical to baseline in `📓️example-geometry-tests-2026-09-09.md`). Not procedural-specific; no new literal dep introduced by this lane's oracle registrations. |
| `verify interactivity` (+`tool-jobs`/`apps`/`apps --actions`) | `bun ./📜️script.ts verify interactivity [tool-jobs\|apps[\|--actions]]` | Not independently re-run for procedural since 2026-09-09; `apps` includes the launch-coverage check (`interactivityAllAppLaunchCoverageFailures`, `📜️script.ts:8429`) that generation3d's three launch variants (react/wgpu-wasm/wgpu-native) satisfy per §5. |
| `verify taxonomy [inventory\|plan\|apply\|verify]` | `bun ./📜️script.ts verify taxonomy verify` | **Fixed 2026-09-10 11:47** (`📓️taxonomy-fix-2026-09-10.md`, `status.md:99`): procedural 650 → 22 violations (18 assembly codec bounds, 4 generated test-host adapters), gen3d 328→4, gen2d 228→2; repo-wide 19 464 → 2 123; **`verify taxonomy` abort cleared**. Restage was needed after (moved dirs) — folded into whatever the next restage picks up. |
| `verify rust-warnings [native\|wasm32-wasip2\|wasm32-unknown-unknown]` | `bun ./📜️script.ts verify rust-warnings wasm32-wasip2` | **New debt, unfixed** (`status.md:540-541`, ~2026-09-10 21:00): `warning: function encode_fault_pack is never used` (fault-arm lane) + an unused `new` in the `framework_reserved_job!` macro, in the shared `🔌️plugin/🦀️.rs` this lane also touches — will fail the gate until patched; explicitly deferred, not this audit's remit. |
| `plugin-registry:generate` / `:check` | `bun nx run @semio-tech/plugin-registry:generate` then `:check` | **generate: pass** (last run 2026-09-09 23:53, `59 plugin crates, 60 playgrounds, 47 framework packages`). **check: catalog-staleness phase pass** after generate; **taxonomy-tree-violations phase** was red repo-wide pre-fix, cleared by the 09-10 taxonomy lane above — not re-verified live since (would need a `check` re-run, out of scope: builds cargo). |
| launch.json ↔ seed coverage | folded into `verify interactivity apps` above | seed (`.vscode/🧩️launch.seed.jsonc`) and generated `.vscode/launch.json` regenerated + verified 2026-09-09 (`📓️launch-entries-2026-09-09.md`); re-checked live this audit (§5) — still in sync, no drift since. |

None of these were re-run in this audit (task forbids starting cargo/nx builds); all rows above are read
from prior reports or, for §5's launch names, a live but build-free `grep` against the current
`.vscode/launch.json`/`.claude/launch.json`.

## 5. Launch entries for procedural 3d (verbatim, live-checked 2026-09-12)

All names below were re-grepped from the current `.vscode/launch.json` and `.claude/launch.json` just now
— unchanged from `📓️launch-entries-2026-09-09.md`, no drift since. Nothing is missing per CLAUDE.md's
"register all executable commands, following existing order/grouping/naming" rule — every row that
audit added/renamed is present, and the ten-row Nx-inferred set (`🖥️launch.ts:190-247`) is complete for
`generation3d`.

**`.vscode/launch.json`** — ten Nx-inferred rows for `generation3d`:
```
🎮️generate🧩️generation3d session
🎮️prepare🧩️generation3d⚛️react dev
🎮️prepare🧩️generation3d⚛️react release
🎮️build🧩️generation3d⚛️react release
🎮️activate🧩️generation3d⚛️react dev
🎮️activate🧩️generation3d⚛️react release
🎮️serve🧩️generation3d⚛️react dev
🎮️serve🧩️generation3d⚛️react release
🎮️dev🧩️generation3d⚛️react dev
🎮️dev🧩️generation3d⚛️react release
```
plus two window-transient test rows: `🧪️generation3d-preview-window-transient`,
`🧪️generation3d-preview-window-transient-native`.

Hand-authored / example-pinned `procedural` rows (react, wgpu-wasm, wgpu-native, build, describe, check —
all present):
```
🛠️dev🔧️procedural🏙️3d⚛️react
🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react
🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm
🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm
🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native
📦️build🔧️procedural🏙️3d
📦️build🧩️procedural⚙️component-dev / ⚙️component-release / ⚙️materialize-dev / ⚙️materialize-release
📦️build🦀️@semio-tech/procedural-plugin
🔎️check🦀️@semio-tech/procedural-plugin
🧪️test⏱️procedural🧊️boot-deadline
🧪️test🌀️procedural📚️examples
🧪️test🚪️procedural🧊️close-ladder
```
(generation2d's mirror set — `🛠️dev🔧️procedural🩻️2d⚛️react` etc. — also present, out of this ticket's
scope but confirms symmetry.)

**`.claude/launch.json`** (hand-maintained Browser-pane config, not generated):
```
procedural3d-react          bun nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev, port 6018, SEMIO_RENDERER=react
procedural3d-wgpu           bun nx run @semio-tech/framework-os-dev:dev -- generation3d, port 6118, SEMIO_RENDERER=wgpu
procedural3d-react-attach   attach-only, url http://localhost:6018, port 6018
```

## 6. Staged/uncommitted diff at time of audit

**Live/moving target — auto-commits land mid-session.** Two snapshots taken minutes apart disagreed (see
below); treat the second as authoritative for "what's currently uncommitted."

- First check (~01:05 CEST): `git diff --cached --stat` restricted to `✏️s/🔌️plugins/🌀️procedural`
  `🧰️framework` → **73 files changed, 4475 insertions(+), 591 deletions(-)**; repo-wide `git diff --cached
  --stat` → **151 files, 17597(+)/971(-)**.
- Second check (01:09:02 CEST): **`git status --short -- ✏️s/🔌️plugins/🌀️procedural` returns nothing** —
  clean. Repo-wide `git diff --cached --stat` → only 4 unrelated ticket-note files (this ticket's own
  `status.md`, `console-dump-probe.mjs`, `🎫️ticket.json`, plus one PUZZLE-3D-END-TO-END probe file), 99(+)/35(-).
- **Cause, verified**: an auto-commit landed at **2026-09-12 01:07:58 CEST**, commit `989582baab2215fcb53fca775d18859064465d29`.
  `git show --stat 989582baab -- ✏️s/🔌️plugins/🌀️procedural 🧰️framework` reproduces the exact 73-file,
  4475(+)/591(-) diff from the first snapshot, byte-for-byte — the first snapshot's staged changes were
  swept into this commit between my two checks, exactly the "the auto-commit will pick it up; that's
  fine" scenario the brief anticipated.

**Net effect for the coordinator**: as of now, `HEAD` (`989582baab`) already contains the generation3d
editor/tick-addressing/fixture/plugin.rs/ShellHost/World3dHost/PluginRuntime/Interpreter changes; nothing
is left uncommitted under procedural. The served wasm (§1, staged 15:51:26) predates all of it (source
mtimes 15:59–16:09, now committed) — **restage is the only remaining step to make this commit observable
on 6018**, not a commit or a stage operation.

## Files referenced (none written outside this report)

- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/{📋️project.json,📜️script.ts}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/{📜️script.ts,🔌️vite-plugins.ts,⚙️vite.config.ts}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/*` (served)
- `Cargo.toml:474-490`, `.cargo/config.toml`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/{contract.ts,🦀️.rs}`
- `.vscode/launch.json`, `.claude/launch.json`
