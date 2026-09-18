# 🧊️ Wgpu renderer — build/serve/test health + theme parity audit

Lane: BUILD / SERVE / TEST HEALTH and THEME/STYLING parity. Read-only (no source files touched).
Repo MCP failed to connect this session (`CONNECTION_CLOSED` on both `repo` and `semio`); this report
lives only on disk. Raw compiler output captured under `🗑️generated/wasm-check.txt` (2 755 lines) and
`🗑️generated/ui-wgpu-check.txt` (112 lines) — this file summarizes, it does not repeat them line for
line.

---

## 1. Browser build health

### 1.1 `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going`

**Result: clean. 0 errors, 243 warnings. `Finished dev profile [unoptimized] target(s) in 8m 20s`.**
Full log: `🗑️generated/wasm-check.txt`.

No crate in the dependency graph failed to compile for the wasm32-unknown-unknown target — the wgpu
renderer's wasm build is currently healthy. Warnings by crate (from the `N warnings generated` summary
lines):

| crate | warnings |
|---|---|
| `semio-framework-os-renderer-wgpu` (the target itself) | 52 (39 auto-fixable) |
| `semio-s-artifact-puzzle-3d` | 103 (96 auto-fixable) |
| `semio-framework-plugin` | 40 (34 auto-fixable) |
| `semio-framework-os-flow` | 4 |
| `semio-s-artifact-puzzle-2d` | 5 |
| `semio-s-artifact-puzzle-5d` | 3 |
| `semio-framework-os-infinite` | 5 |
| `semio-framework-replication` | 6 |
| `semio-framework-artifact-flow-flow` | 2 |
| `semio-framework-ui` | 2 |
| `semio-framework-job` | 2 |
| `semio-framework-trace`, `semio-framework-ui-runtime`, `semio-framework-os-kernel`, `semio-framework` | 1 each |

Warning categories, whole log (243 total): 169 `unnecessary qualification` (mechanical, one crate —
almost certainly `cargo fix --lib -p <crate>` alone would clear most of them), 15 `unused import`, 12
`function … is never used`, 7 `unused variable`, 5 `method … is never used`, 3 `constant … is never
used`, 2 `use of deprecated method … fetch_update: renamed to try_update` (both in
`semio-framework-replication`/`semio-framework-trace` neighborhood, not in the wgpu target itself),
plus one-offs (`variant never constructed`, `struct never constructed`, `type more private than the
item it's returned from`, `unnecessary parentheses`, `redundant pattern binding`, `unused extern
crate`).

Dead-code warnings worth flagging as *possible parity debt* (things a React-parity pass might expect
to be reachable but currently are not, inside the wgpu-specific renderer/shell/dock/scenes files —
`🧊️renderer/🦀️.rs`, `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`,
`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`):
- `dock_tabs_from_ids`, `mode_layout_stacks_v1`, `dock_seed_active_window_id_v1`,
  `chord_carries_accelerator_v1`, `reserved_shell_chords_v1`, `KEYBINDING_UNOWNED_CODE` — a whole
  unused window-scope-stack-v1 / reserved-chord subsystem in Shell (lines ~10768–10870) that looks
  like a half-landed keybinding-ownership feature.
- `SceneDragMode::InkResize` variant never constructed — a drag mode declared but never entered.
- `ShellFindItems::close_step` never used, `ShellDocumentRetirementSlot{epoch,generation,surface}`
  fields never read — retirement/close-step plumbing present but currently dead in this crate (compare
  with the `FixedOperationRegistry`/close-step conventions noted elsewhere in the ticket history).

None of this blocks a build; it is signal for whoever owns Shell/Dock/Scenes parity waves to check
whether these are stubs waiting for a caller or genuinely obsolete.

### 1.2 `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` (native)

**Result: clean. 0 errors. `Finished dev profile [unoptimized] target(s) in 37.61s`.**
Full log: `🗑️generated/ui-wgpu-check.txt` (112 lines — this is a much smaller closure since it's the `ui`
crate alone with its dependency chain, not the whole os-renderer-wgpu binary).

Only `semio-framework-ui` itself produced warnings from its own code (2: an unused import
`tree_section_header_height` in `🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:29` and a dead constant `PANEL_HEADER`
in the same file at line 42 — both trivial, both isolated to paint.rs). The rest of the log is
transitively-checked dependency warnings (`semio-framework-replication`'s `unnecessary qualification`
×4, one `fetch_update`→`try_update` deprecation each in `trace` and `os-kernel`/neighboring crates) —
none in `semio-framework-ui` or its wgpu target code.

**Conclusion for item 1: both the wasm and native wgpu-adjacent compilation targets are green right
now.** There is no build-health blocker to React parity work; the backlog is warning cleanup, not
errors.

---

## 2. Serve pipeline

### 2.1 The Nx target chain

`nx run @semio-tech/framework-os-dev:dev -- <plugin>` (also reachable as `@semio-tech/framework-renderer-wgpu:dev`,
rewritten by `resolveNxInvocation` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts`) resolves to
`@semio-tech/framework-os-dev:dev-<plugin>-wgpu-<profile>`, which `dependsOn`
`activate-<plugin>-wgpu-<profile>`.

**`activate-s-wgpu-dev` (and every `activate-<plugin>-wgpu-<profile>` target) is not a hand-authored
Nx target.** It is generated per plugin/profile by the `createNodes`-style plugin at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (registered in `nx.json` `plugins[2]`), around
line 1035:

```js
result[`activate-${playground.variant}-wgpu-${profile}`] = {
  cache: false,
  outputs: [`{projectRoot}/dist/runtime/wgpu/${profile}/${playground.variant}`],
  inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
  dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
  options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
};
```

`prepare-<variant>-wgpu-<profile>` (line 1042) is what actually forces the wasm build and generators:
its `dependsOn` includes `@semio-tech/plugin-registry:session-<variant>`,
`@semio-tech/framework-plugin-web:support-<profile>`, `semio-framework-os-infinite:fonts`,
`<wgpuProject>:wasm` (or `:wasm-release`), `<wgpuProject>:generate-browser-boot`,
`<wgpuProject>:generate-frame-worker`, and one `materialize-<profile>` per component in the runtime
closure. `playgrounds` themselves come from every Cargo.toml's `[package.metadata.semio].playground`
array (discovered by scanning every `Cargo.toml` under the workspace, skipping `.🧬semio`/`compose`/
generated dirs) — this is genuinely metadata-driven, not enumerated by hand anywhere.

`serve`/`dev` targets on both `@semio-tech/framework-os-dev` (project.json line ~124) and on the wgpu
package's own project.json (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json:390-410`)
both `dependsOn: ["@semio-tech/framework-os-dev:activate-s-wgpu-dev"]` and run the SAME command:
`bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve s dev` — the os-dev
package is the canonical caller, the wgpu package's own `serve`/`dev` are a thin duplicate pointed at
the same script.

### 2.2 The server script

`🌐️server/📜️script.ts` (`ServeScript`) resolves the playground row (`PLAYGROUND_BUILD_TARGETS`,
generated at `🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts`), picks a port
(`S_OS_PORT` env or `playground.ports.wgpu`), sets `SEMIO_PLUGIN`/`SEMIO_RENDERER=wgpu`/
`SEMIO_BUILD_MODE`, and calls `serveVite` with `🎚️config/🟦️.ts` as the Vite config (native config
loader — no separate bundling step).

`🎚️config/🟦️.ts` builds a `WgpuBrowserConfiguration` from **already-completed artifact directories**,
never compiling anything itself:
- `compilerRoot` = `../📦️packages/🦀️rust/dist/wasm-<profile>` (the wasm-bindgen output —
  `semio-framework-os-renderer-wgpu.js` + `..._bg.wasm`, produced by `🏗️compiler/🌐️wasm/📜️script.ts`)
- `bootRoot` = `../🚀️browser-boot/🤖️generated` (a single `🟨️.js`, produced from
  `🚀️browser-boot/🟦️.ts` by the `generate-browser-boot` target)
- `workerRoot` = `../🎞️frame-worker/🤖️generated` (single `🟨️.js`, `generate-frame-worker` target)
- `moduleRoot`/`extensionRoot` = per-plugin/per-extension module dirs under the **os-dev package's own**
  `dist/runtime/wgpu/<profile>/<variant>` tree (`developmentRuntimeRoot`, `pluginModulesRoot` from
  `🧑‍💻dev/♻️activation/🟦️.ts`)
- `fontRoot` = `♾️infinite/📦️packages/🦀️rust/dist/fonts`
- `reloadFile` = `<runtime>/activation/<ACTIVATION_RECEIPT_FILE>` — the server watches this file and
  does a Vite full-reload whenever Nx's `activate-*` target rewrites it, which is how "serve stays up,
  Nx recompiles in the background, browser reloads on completion" works (`completedArtifactReload` in
  `🌐️server/🟦️.ts`).

`createWgpuBrowserConfig` throws **before serving** if any of
`{compilerRoot}/semio-framework-os-renderer-wgpu.js`, `..._bg.wasm`, `{bootRoot}/🟨️.js`,
`{workerRoot}/🟨️.js`, `{fontRoot}/<FONT_ASSET>` is missing — so a cold serve with no prior `prepare`/
`activate` run fails fast and loud rather than serving a 404 at runtime. There is also a
`wgpu-serve-only` plugin that throws if the Vite `command` is ever `build` (`"Build the finite WGPU
wasm target through Nx"`) — this dev server is explicitly serve-only, no bundling escape hatch.

`🌐️.html` is an 18-line static shell: `<script type="module" src="/🚀️boot.js/🟨️.js">`, a `#root` div,
`background:#001117` (== the palette's `--color-dark` hex, matches `Theme::dark().background`), and
`canvas, #semio-wgpu-canvas { outline:none; border:none }`. No React, no CSS bundle reference — the
whole visual surface is wgpu-painted; this file itself carries zero theme risk since it contributes no
visible chrome of its own.

### 2.3 Compiler / boot / frame-worker

- `🏗️compiler/🌐️wasm/📜️script.ts` — `wasm` / `wasm-release` Nx targets, `dependsOn: [workspace:deps-cargo,
  workspace:deps-trunk, workspace:deps-wasm-opt]`, outputs `dist/wasm-dev` / `dist/wasm-release`.
- `🏗️compiler/🦀️native/📜️script.ts` — native binary build (`nativeRendererBinary`), backs
  `native-build`/`native-build-release` and the `[[bin]] name = "semio-wgpu-native"` target declared in
  `📦️packages/🦀️rust/Cargo.toml:82-83`.
- `🚀️browser-boot/🟦️.ts` + `generate-browser-boot` target → `🤖️generated/🟨️.js` (**present on disk**).
- `🎞️frame-worker/🟦️.ts` + `🏗️builder/🟦️.ts` + `generate-frame-worker` target → `🤖️generated/🟨️.js`
  (**present on disk**).
- `⌨️native-entrypoint/📜️script.ts` — `RunScript`/`ScaleScript`; reads a completed
  `dist/runtime/native/<profile>/<variant>/🔣️runtime.json` + `.nx-artifact.json` (never compiles),
  strips credential-shaped env vars before spawning the child (`nativeRunnerEnvironment` — drops
  `S_USER`/`VITE_S_USER`/`S_HUB_URL` and anything matching
  `TOKEN|SESSION|CREDENTIAL|BEARER|CAPABILITY|AUTHORIZATION|COOKIE`), and can start a local asset
  server (`startAssetServer`) for playgrounds that declare `assets`.

### 2.4 What's on disk right now (assessed, not started)

```
🧰️framework/…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/dist/
  native-dev/   (present, populated)
  wasm-dev/     (present, populated — mtime 2026-09-16 02:36, i.e. from a recent activation)
🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/dist/
  runtime/wgpu/dev/
    generation3d/  {activation/, extensions/}
    puzzle3d/      {activation/}
    gis2d/         {activation/}
  build-energy-react-release/, build-fem3d-react-release/  (react-target build outputs, unrelated)
  .staging/
```
No `wasm-release`/`native-release` dist dirs currently present (only `-dev` profiles have been
activated), and only 3 of the registered wgpu playgrounds (`generation3d`, `puzzle3d`, `gis2d`) have a
`dev/runtime/wgpu/dev/<variant>` directory — `puzzle3d` and `gis2d` are missing an `extensions/` dir
that `generation3d` has (their runtime closures may simply declare no extensions; not necessarily a
gap). `s`/`puzzle`/`puzzle5d`/`architect`/`flow` etc. (see `.claude/launch.json` wgpu entries below)
have **no** activated dev runtime directory right now — a cold `nx run @semio-tech/framework-os-dev:dev -- <variant>`
for any of those would need to run `prepare`→`activate` first (expect the wasm/native compile + font +
materialize chain, not just a few seconds).

### 2.5 Launch configurations

`.claude/launch.json` (mirrored, with per-example-battery expansion, into `.vscode/launch.json`, 131 KB,
64 `"wgpu"` entries) currently names these wgpu dev entries (`nx run @semio-tech/framework-os-dev:dev -- <variant>`,
`SEMIO_RENDERER=wgpu`):

| name | port | variant |
|---|---|---|
| procedural3d-wgpu | 6118 | generation3d |
| puzzle3d-wgpu | 6113 | puzzle3d |
| puzzle5d-wgpu | 6114 | puzzle5d |
| gis2d-wgpu | 6140 | gis (2d) |
| puzzle5d-native | — | `@semio-tech/framework-renderer-wgpu:native -- puzzle5d` |

`.vscode/launch.json` additionally has per-example native/wasm debug configurations for cad, dag,
mathematical-equation, architect-program, flow, imperative-procedure, sequence, lowpoly, layout (each
with a `…🧊️wgpu🌐️wasm` and `…🧊️wgpu🖥️native` pair) — these look auto-generated from the playground
registry (ticket `26/08/05/LAUNCH-JSON-GENERATOR-FROM-PLAYGROUND-REGISTRY` matches this shape).

### 2.6 Native path

`@semio-tech/framework-renderer-wgpu:native`/`:native-release` route (via the bootstrap router) to
`prepare-<variant>-native-<profile>` → `<wgpuProject>:native-build[-release]` → the `semio-wgpu-native`
binary, then `run-<variant>-native-<profile>` runs it via `⌨️native-entrypoint/📜️script.ts`'s
`RunScript`, which starts a `winit`-backed window (crate deps confirm `winit` under the `wgpu-engine`
feature in `semio-framework-ui`'s Cargo.toml) reading the same completed-artifact contract as the
browser path (no separate theme/metrics source — same `ui_styling` crate, same `Theme::dark()`/
`light()`).

### 2.7 Most recent boot evidence (since 2026-08-01)

The freshest, most complete wgpu-boot trail is ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/` (`generation3d`, ports 6118/6027/6013/
6021-6028 across its many sub-lanes), with dated status notes through **2026-09-15 21:50** — the newest
ticket folders (`🌙️09/☀️16`, `🌙️09/☀️17`) contain nothing newer that documents an actual wgpu boot.

State as of the last entries (09-15, `📓️status.md` tail):
- The wgpu serve on generation3d (`:6118`/`:6027`/recycled `:6013`/`:6024` etc.) was live and being
  actively battery-tested most of that day; **closing state was green**: `journey 25/25`, `interact
  58/58`, `role-switch 9/9`, `flow-window 3/3`, **0 page errors** on the final gates
  (`hot-swap-board-remount`, `retirement-ladder-credit`, `concurrent-patch-intake` lanes' closing
  probes).
- Major cross-renderer defects found and fixed that session, all wgpu-bridge-specific (React was
  already correct in each case) — relevant to this ticket's "functional parity" goal directly:
  - **Selection never rendered** (`📓️wgpu-selection-roundtrip-2026-09-15.md`): a reserved-tool job
    completion frame answers `in_reply_to: 0` (no caller); React's `deliverJobCompletionTurn` parks
    such frames on a LEFTOVER lane and peels them back, but the wgpu bridge's `drainSpawnedJobs`/
    `stashLeftoverHostEffects` explicitly dropped shell frames from that lane — the fix makes
    `wgpuInvocationFromFrames` fold the leftover lane the same way `PluginRuntime` does. Confirms the
    wgpu **bridge** (not the renderer/theme) was the parity gap for interaction results.
  - `wgpu-host-settle-pump`, `wgpu-mesh-oracle`, `wgpu-wheel-zoom-a11y-live`,
    `wgpu-server-input-present`, `wgpu-worker-boot`, `wgpu-shell-boot-silence`,
    `wgpu-catalogue-fixed-map`, `wgpu-dock-layout-world3d`, `wgpu-tree-row-hit-test`,
    `wgpu-intake-budget`, `wgpu-extension-dispatch`, `wgpu-path-audit`, `wgpu-ui-turn`,
    `wgpu-boot-divergence`, `wgpu-boot-watchdog`, `wgpu-playground-boot`, `wgpu-example-chain` — this
    same ticket's earlier (09-09 through 09-14) sub-reports are almost entirely wgpu-runtime/bridge
    parity fixes (input wiring, worker boot ordering, dock layout, tree hit-testing), i.e. this ticket
    (`WGPU-RENDERER-REACT-PARITY`) is picking up work immediately downstream of a very active, very
    recent (this week) parity effort — most of the *shell/runtime* gaps were already being closed;
    remaining explicit open items at 09-15 close: **why R3F reports a hover miss where its raycast
    hovers** (React-side, not wgpu), **`ToolRunProvisionalIdsContext` has no production provider**, and
    **`♻️hot-swap.json` watcher not registered in the dev vite config**.
- `📓️2026-09-09-wgpu-coverage-audit.md` (ticket `26/09/02/PUZZLE-3D-END-TO-END`) is the next most recent
  wgpu-specific coverage audit for a different plugin (puzzle3d) if broader-than-generation3d evidence
  is wanted.

**Bottom line for item 2**: the serve pipeline is fully wired, artifact-gated (fails fast on missing
compiled output, never silently serves stale/absent code), and was proven live and green on
generation3d 2 days before this audit. It was NOT started or exercised by this audit (per instructions).

---

## 3. Tests

### 3.1 ui wgpu unit tests (`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-*`, 30 case dirs)

**None of these are declared as `[[test]]` entries in any `Cargo.toml`.** `semio-framework-ui`'s
manifest (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`) has zero `[[test]]` sections. They
are wired directly into the crate's own source modules via `#[cfg(test)] #[path = "…"] mod tests;` (23
cases) or `include!("…")` inside an existing `#[cfg(test)] mod tests { … }` block (6 "component-*"
cases, all inside `🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`), e.g.:

```rust
// 🐚️shell/🦀️.rs:270-272
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-shell-unit/🦀️.rs"]
mod tests;
```

Full map (host file → test case dir): `input-unit`←`📥️input`, `tree-unit`←`🌳️tree`,
`arena-unit`←`🏟️arena`, `scene-slots-unit`←`📨️scene_slots`, `cursor-unit`←`👆️cursor`,
`draw-unit`←`🖍️draw`, `draw-types-selection-marquee`←`🖍️draw/🏷️types`, `engine-unit` +
`engine-retained-document-hostile-fixtures`←`⚙️engine`, `text-unit`←`📝️text`, `shell-unit`←`🐚️shell`,
`paint-unit`←`🖌️paint`, `mounted-layout-unit`←`📌️mounted_layout`,
`icon-name-value-icon-name-value-round-trip`←`🔣️icon-name/🧾️value`, `events-unit`←`⚡️events`,
`prepared-unit`←`🎟️prepared`, `locale-terminology-value-…-round-trip`←`🌐️locale-terminology/🧾️value`,
`layout-unit`←`🧮️layout`, `reconcile-unit`←`🔀️reconcile`, `label-localized-label-value-round-trip`←`🏷️label`,
`accessibility-projection`←`♿️accessibility`, `action-unit`←`🎬️action`, `flex-legacy-layout-job` +
`flex-unit`←`📐️flex`, `gpu-prepared-present`←`🧊️gpu`, `component-layout-{layout-wire-format,value-round-trip}`,
`component-role-chrome`, `component-utilities-utility-node-wire-format`,
`component-ui-{ui-node-wire-format,value-round-trip}`←`🧩️component`.

Because every one of these is behind `#[cfg(test)]`, **item 1's `cargo check --lib` never compiles them
at all** — `--lib` never activates `cfg(test)`. Compiling them requires `cargo check|test --lib --tests`
(or `--all-targets`) on `semio-framework-ui`. This is a known sharp edge here already: the twin
os-renderer-wgpu test file (§3.2) carries a doc comment recording a **real incident** where a missing
`#[path]` target wedged `cargo test -p semio-framework-os-renderer-wgpu --lib` for the **whole crate**
with "couldn't read …: No such file or directory" — i.e. a dangling `#[path]` to a nonexistent test
file is a hard compile failure for the whole crate's test target, not a soft warning. None of the 30 ui
directories are currently dangling (all 30 have their referenced `🦀️.rs` file present).

### 3.2 os renderer engine wgpu tests (`🧰️framework/🛍️products/💻️os/…/🎯️targets/🧊️wgpu/🧪️tests/`, 4 dirs)

- `🎮️wgpu-browser-input-wire/🦀️.rs` — **populated**, wired from `🎮️input-wire/🦀️.rs:172-173` the same
  `#[cfg(test)] #[path=…]` way. Its own header comment documents the incident above and marks itself an
  intentionally-empty placeholder ("Created empty … purely so the crate's test target builds again")
  pending the real assertions from a `wgpu-server-input-present` lane (see §2.7 — that lane already
  landed 2026-09-13, so this stub may now be stale/supersedable — worth a follow-up check by whoever
  owns test coverage, not fixed here per the read-only mandate).
- `🎮️browser-interactive-job-port/`, `📨️browser-frame-transport/`, `🧩️package-integration/` — **all
  three are genuinely empty directories** (confirmed via `os.walk`, not a shell-glob artifact) and are
  **not referenced by any `#[path]`/`include!`** anywhere in the crate — they are unwired stubs, not
  dangling references, so they carry no compile risk today, but also currently assert nothing.
- `🎚️config/🟦️.ts` in the same `🧪️tests/` dir is unrelated TS test tooling config, not a Rust case.

### 3.3 Taxonomy-driven `.feature` test plugin — does NOT own these cases

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` (the other Nx plugin registered in `nx.json`,
`createNodesV2: ["**/*.feature", …]`) generates one virtual Nx project + 4 level targets (`quick`/
`long`/`exhaustive`, cumulative) per **`.feature`-file-bearing** case directory under any `🧪️tests/`
folder, taxonomy-cased. None of the 34 directories audited above contain a `.feature` file (they hold a
bare `🦀️.rs`), so **none of them are Nx-visible test cases today** — they run only as part of whatever
invokes `cargo test`/`cargo nextest` directly on `semio-framework-ui` /
`semio-framework-os-renderer-wgpu` with `--tests`, not through a generated Nx `test-quick`/`test-long`/
`test-exhaustive` target. This is a real gap if the intent is "every wgpu unit test is independently
selectable/cacheable through Nx" — right now they are not; they are ordinary `#[cfg(test)] mod tests`
blocks compiled as part of the crate's monolithic `--tests` target.

### 3.4 What item 1's checks did NOT prove

Because both checks in §1 were `--lib` (no `--tests`/`--all-targets`), **none of the 34 test case files
above were compiled by this audit**. A follow-up `cargo check -p semio-framework-ui --features
wgpu-engine --tests` / `cargo test -p semio-framework-os-renderer-wgpu --lib --no-run` was explicitly
out of scope ("do not run the full test suite") and was not attempted.

---

## 4. Theme/styling parity

### 4.1 The source of truth is already shared — this is the headline finding

Both renderers' colors and metrics are **generated from the same JSON**,
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` (`colors`, `spacing`, `metrics`, `radii`, `strokes`,
`levels`, `presence`, `appearances`):
- → CSS custom properties for React: `🎨️palette/🎨️.css` (`@theme { --color-*, --font-*, --spacing-* }`,
  generated — file header literally says "Generated from framework/ui/styling/🔣️.json — run `bun
  ./📜️script.ts generate`"), consumed via `@import` chains ending at
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎨️.css`.
- → a Rust `ui_styling` crate (`ChromePalette`, `CHROME_LIGHT`/`CHROME_DARK`, `metrics::{chrome, dom,
  typography}`, `radii`, `strokes`, `levels`) consumed by
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs`'s `Theme::light()`/`Theme::dark()`
  (`from_chrome`). The Rust projection itself is codegen'd at build time (no committed `.rs` under
  `🤖️generated/`, only the CSS/TS projections are checked in — `🤖️generated/🔤️tokens/🟦️.ts` and 3 CSS
  files) — consistent with the crate importing `super::generated::{ChromePalette, CHROME_DARK,
  CHROME_LIGHT, …}` from a module that isn't a checked-in source file.

Because of this, **most tokens are correct by construction, not by discipline** — there is structurally
no way for `Theme::dark().navbar_height` to drift from React's navbar height, since both are `9 ×
uiSpacingCompactPx` (`chrome.navbarHeightUiSpacing = 9.0`, `chrome.uiSpacingCompactPx = 3.2`) read from
the identical JSON key. Verified concretely:

| token (🔣️.json path) | value | React (CSS) | wgpu (`Theme`) | parity |
|---|---|---|---|---|
| `spacing.compact` / `metrics.chrome.uiSpacingCompactPx` | `0.2rem` / `3.2px` | `--spacing-compact: 0.2rem` | `chrome_px(1.0)` = 3.2px | ✅ same source |
| `metrics.chrome.navbarHeightUiSpacing` | `9.0` → 28.8px | (Tailwind class off `--spacing-compact`) | `navbar_height` | ✅ |
| `metrics.chrome.controlHeightUiSpacing` | `7.0` → 22.4px | idem | `control_height` | ✅ |
| `metrics.chrome.panelHeaderHeightUiSpacing` | `7.0` → 22.4px | idem | `panel_header_height` | ✅ |
| `metrics.chrome.footerHeightUiSpacing` | `9.0` → 28.8px | idem | `footer_height` | ✅ |
| `metrics.dom.treeRowUiSpacing` | `7.5` → 24px | `h-workbench` (`--size-workbench`) | `tree_row_height` | ✅ (comment in theme.rs names the React class explicitly) |
| `metrics.dom.treeIndentPerLevelUiSpacing` | `3.125` → 10px | React's tree gutter step | `tree_indent_per_level` | ✅ |
| `metrics.dom.treeToggleUiSpacing` | `4.375` → 14px | React's expand/collapse gutter | `tree_toggle_width` | ✅ |
| `metrics.dom.layoutPanelMinUiSpacing`/`MaxUiSpacing` | 46.875 / 150.0 → 150 / 480px | panel min/max width | `panel_min_width`/`panel_max_width` | ✅ |
| `typography.text{2xs,xs,sm,base,lg}Px` | 9.6/11.2/12.8/14.4/16.0 | Tailwind text-* utilities | `font_size_small`(xs)/`font_size_body`(sm)/`font_size_emphasized`(base) | ✅ (wgpu only names 3 of the 5 scale steps — `text2xs`/`textLg` have no `Theme` field; fine if nothing in the wgpu chrome uses those two sizes, otherwise a gap) |
| `radii.chrome` | `0.0` | `border-radius: 0` (8 sites in `🖌️ui/🎨️.css`, chrome/panel/dialog selectors) | `border_radius: radii::CHROME as f32` = 0.0 | ✅ square chrome by design, confirmed on both sides |
| `levels.glassAlphaStep` / `glassBlurStepPx` / `glassSaturate` | 0.12 / 8 / 1.45 | `--glass-alpha-step: 0.12; --glass-blur-step: 0.5rem (8px); --glass-saturate: 1.45` (`🖌️ui/🎨️.css:843-949`, `calc(1 - k*step)`/`calc(k*step)` per level) | `Theme::glass()`: `alpha = 1 - k*GLASS_ALPHA_STEP`, `blur_px = k*GLASS_BLUR_STEP_PX`, `glass_saturate` | ✅ identical formula, identical constants |
| `colors.warning` | `#fccf05` | `--color-warning: #fccf05` | `warning: Rgba::from_srgb8(252,207,5,255)` (0xFCCF05) | ✅ exact hex match |
| `colors.dark` | `#001117` | `--color-dark: #001117`; `chrome.base = {token:"dark"}` in dark appearance | `Theme::dark().background` (via `from_chrome(&CHROME_DARK)`); server `🌐️.html` also hardcodes `background:#001117` for the pre-boot canvas | ✅ (the `.html` literal is a fine intentional duplicate — it paints before any JS/WASM runs) |

### 4.2 Real mismatches / unparitied constants (hand-written literals bypassing the token pipeline)

`🎨️theme/🦀️.rs`'s `from_chrome()` has six `Rgba::new(...)` / one `Rgba::from_srgb8(...)` calls that do
**not** read from `ui_styling` at all — these are the only places wgpu chrome color can silently drift
from React, because nothing regenerates them:

| field | value (hardcoded) | nearest 🔣️.json token | verdict |
|---|---|---|---|
| `error` | `Rgba::new(0.95, 0.35, 0.35, 1.0)` ≈ `#F35A5A` | `colors.danger = #a60009` | **mismatch** — wgpu's "error" red is a soft, light red; the palette's actual `danger` token is a near-black-red. If React error/danger UI uses `--color-danger` (likely, given the token exists specifically for it) the two renderers show visibly different error colors. |
| `success` | `Rgba::from_srgb8(36,158,91,255)` = `#249E5B` | `colors.success = #7eb77f` | **mismatch** — wgpu's success green is darker/more saturated than the palette's `#7eb77f`. |
| `progress` | `Rgba::from_srgb8(67,132,245,255)` = `#4384F5` | *(no `colors.*` token named progress/info-blue; `colors.info = #dbbea1` is a tan, not blue)* | **no named-token equivalent at all** — this blue is wgpu-only; unclear what (if anything) React paints for the same semantic "still running" state, or whether it's a different blue. Needs a design-source check outside this audit's file scope. |
| `checker_light` / `checker_dark` | `Rgba::new(0.85,0.85,0.85,1.0)` / `Rgba::new(0.72,0.72,0.72,1.0)` | none | No `checker` string anywhere in `🖌️ui/🎨️.css` or the React renderer target — this checkerboard (almost certainly a transparency-preview backdrop) is a wgpu-only concept with no CSS counterpart to compare against; React likely does an inline `background-image: repeating-conic-gradient(...)` per component instead (the Storybook `GlassContent` story's own floor uses exactly that pattern) rather than a theme token, so this may not be a true "mismatch" so much as "not represented as a token on either side." |
| `diagram_stroke` / `diagram_seam` / `diagram_accent` / `diagram_accent_fill` | `Rgba::new(0.2,0.55,0.95,0.95)` / `(0.95,0.45,0.2,0.95)` / `(0.25,0.45,0.65,0.9)` / `(0.25,0.35,0.55,0.8)` | none | Four more hand-picked colors with no `🔣️.json` key and nothing matching in the React CSS grep. Likely backing some wgpu-only diagram/graph paint path; if the React DAG/flow renderer (`♾️infinite/🌍️world/🎨️r3f`, sourced into the same `.css` via `@source`) paints edges/seams with different literal colors, this is exactly the kind of silent divergence token-driven parity was supposed to prevent. |
| `control_height_small` | `chrome_px(5.0)` = 16px | none (`metrics.chrome` has no `controlHeightSmallUiSpacing` key) | The *multiplier* `5.0` is a literal inside `from_chrome()`, not sourced from JSON at all — every other `chrome_px(...)` call passes a named `chrome_metrics::…` constant; this one is the only inline magic number in the metrics block. Drift risk: a future edit to the design tokens would not touch this value even if intended to. |

### 4.3 Structural notes

- `AppearanceName` is `{Light, Dark}` only, matching React's `.dark` CSS class toggle (no third
  system-auto variant baked into the palette itself — that decision presumably lives above both
  renderers).
- `Theme::for_name`/`light()`/`dark()` and `presence_color()` also pull from the same generated
  `presence_bar`/`ui_styling::presence` source as `🤖️generated/🚦️palette-presence/🎨️.css`'s 12
  `--presence-N` HSL swatches (`hsl(0..330deg, 68%/72%, 32%/62%)` light/dark) — the wgpu presence ring
  is genuinely the same 12-color cycle, not a re-derivation.
- `glass_mip_level` (mip selection for blurred backgrounds) has no React equivalent to compare — it's
  an implementation detail of how wgpu approximates CSS `backdrop-filter: blur()` via pre-blurred
  mipmaps, invisible at the token level; only worth checking visually, not textually.
- Icon sizes: `dom.icon{Tiny,Small,Base,Large}UiSpacing` (3.75/6.25/7.5/10.0 → 12/20/24/32px) exist in
  the shared JSON but **`Theme` does not surface any of the four as a field** — the wgpu icon renderer
  presumably reads `ui_styling::metrics::dom` directly rather than through `Theme` (not inspected
  further here; flagged for whoever audits `♿️accessibility`/`🔣️icon-name` wgpu code specifically, since
  it's outside a pure theme.rs read).

### 4.4 Storybook wgpu story

`🧰️framework/🛍️products/💻️os/📖️stories/🎭️wgpu/🧪️.story.tsx` (56 lines) defines 4 stories under
`🛠️framework🖥️os/Wgpu` via a shared `WgpuBootHost` component
(`🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx`):
- **Studio** (`plugin: "s"`) — boots the real wgpu renderer for the `s` (studio) program.
- **Puzzle** (`plugin: "puzzle"`) — same, for `puzzle`.
- **ArtifactMissing** (`plugin: "architect"`) — deliberately picks a `pluginId` with no prebuilt web
  artifact, to exercise the "artifact-missing" fallback panel **offline and deterministically** (no
  cargo build triggered — matches §2.2's "throws before serving" contract).
- **GlassContent** — fixed high-contrast checkerboard floor (`#641f45` + a 32px diagonal-stripe
  `linear-gradient`, i.e. exactly the ad-hoc checkerboard pattern noted in §4.2) behind a real
  `WgpuBootHost`, specifically for inspecting the native WGPU window's glass/blur cutouts against a
  busy background — this is the closest thing to an existing visual regression harness for the glass
  system audited in §4.1/§4.3.

`WgpuBootHost` itself: dynamically imports the wgpu package, resolves the Trunk-hashed bundle filename
from the served `index.html`, boots into `#root`; falls back to a "WebGPU unavailable" message when
`navigator.gpu` is undefined (headless Chromium without `--enable-unsafe-webgpu`, Safari, Firefox) —
relevant if any future automated visual-parity script runs this story under Chromium and needs the
right launch flag to actually exercise the wgpu path rather than silently hitting the fallback message.

---

## Recommended work packets

Each packet is independently assignable; file seams are given so two agents can work the same area
without touching each other's files.

1. **Fix the 6 hand-literal theme colors in `theme.rs`** (§4.2).
   - Seam: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs`, `from_chrome()` only.
   - Reference: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` (`colors.danger`, `colors.success`); for
     `progress`/`checker_*`/`diagram_*`/`control_height_small` first determine whether React has ANY
     visual equivalent (grep the React renderer + `♾️infinite/🌍️world/🎨️r3f` for the concepts named
     "progress"/"running" status pill, DAG edge/seam colors, transparency checkerboard) before deciding
     whether to add new named tokens to `🔣️.json` or intentionally keep them wgpu-local with a comment
     explaining why.
   - Acceptance: no `Rgba::new(...)`/`Rgba::from_srgb8(...)` literal left in `from_chrome()` without
     either (a) a `ui_styling::…` token behind it, or (b) a comment naming the deliberate,
     verified-against-React design reason it stays literal.

2. **Give the 34 wgpu unit-test case dirs Nx visibility** (§3.3).
   - Seam: purely additive — either add `.feature` files to route them through
     `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`'s existing taxonomy discovery, or (cheaper)
     add an explicit `test`/`test-wgpu` Nx target on `semio-framework-ui`'s and
     `semio-framework-os-renderer-wgpu`'s own `📋️project.json` that runs `cargo test … --lib --tests`
     scoped to these modules.
   - Reference: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` (`testCaseProjects`, `canonicalCase`)
     for the taxonomy shape; `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-shell-unit/` for a
     concrete example case.
   - Acceptance: `nx show projects` lists a runnable target per case, OR a single new Nx target compiles
     `--tests` for both crates in CI/local without depending on a human remembering the raw `cargo test`
     invocation.

3. **Decide the fate of the 3 empty os-renderer-wgpu test stub dirs + the intentionally-empty
   `wgpu-browser-input-wire` placeholder** (§3.2).
   - Seam: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/{🎮️browser-interactive-job-port,📨️browser-frame-transport,🧩️package-integration,🎮️wgpu-browser-input-wire}/`.
   - Reference: the placeholder's own doc comment names its intended owner
     (`wgpu-server-input-present` lane, ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, landed 2026-09-13 —
     check whether that lane's real assertions should have replaced this file and didn't).
   - Acceptance: each dir either gets real assertions wired via `#[path]`/`include!`, or is deleted with
     a one-line note on why the case was retired (never silently left both empty AND unwired forever).

4. **Warning cleanup pass on the wgpu wasm target crates** (§1.1) — lowest priority, purely hygiene.
   - Seam: run `cargo fix --lib -p semio-framework-os-renderer-wgpu` /
     `-p semio-s-artifact-puzzle-3d` / `-p semio-framework-plugin` for the 169 mechanical
     `unnecessary qualification` fixes (safe, auto-applied); hand-review the `never used`/`never read`/
     `never constructed` dead-code warnings in Shell/Dock/Scenes (§1.1's bulleted list) since several
     look like half-landed features (`WindowScopeStackV1`, reserved-chord ownership, `InkResize` drag
     mode) that a parity wave might actually need to finish rather than delete.
   - Acceptance: `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown
     --keep-going` warning count materially down from 243 with no new errors; each surviving
     Shell/Dock/Scenes dead-code warning has an explicit "keep, unfinished feature" or "remove" decision
     recorded.

5. **Verify the serve pipeline live on the 4 currently-un-activated playgrounds** (§2.4) before
   declaring "serve is healthy" universally — this audit only confirmed the artifact-gating logic and
   the 3 already-activated dev runtimes (`generation3d`, `puzzle3d`, `gis2d`); `s`/`puzzle`/`puzzle5d`/
   `architect`/others have no dev runtime directory on disk right now and were not booted (per this
   ticket's read-only mandate) — a follow-up should run one cold `nx run
   @semio-tech/framework-os-dev:dev -- <variant>` per un-activated playground and confirm the
   `prepare`→`activate`→serve chain completes end to end.
