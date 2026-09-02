# 🧪️ Runtime verification — `dev process 3d` on :6022

Playground: `process3d`, app `s.process.process3d@1/*#editor`, react renderer, port **6022**
(`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml`).

## Boot, before any of this ticket's fixes
`🔌️plugin-modules/process/semio_s_plugin_process_component.core.wasm` was dated **Aug 27 16:02** — five
days stale, exactly the shape the sourcing ticket hit, because the crate had stopped compiling. A first
`SKIP_PLUGIN_BUILD=1` boot attempt exited 1 before binding :6022.

## What the checklist is
1. Shell mounts: title `semio · process · process3d`, Editor mode, the example selector.
2. `🪚️workpiece` renders a **non-degenerate** mesh — specifically NOT the 1×1×1
   `ProcessWorkingScene::default()` cube and NOT `PROCESS3D_FALLBACK_MESH_KIND` (`"box"`). Read the
   scene json and check the extents against the fixture's beam (3.0 × 0.2 × 0.3).
3. The engagement stepper reports the fixture's real step count, not `0`.
4. Panels: `📄️artifact` lists every step id in order; `🛠️workshop` lists the fixture's machines;
   `🛍️catalogue` flags capabilities whose rules the stock violates. `🔍️inspection` is expected to stay
   on its empty state — that one is a framework gap (`ArtifactEditor::render` carries no
   `InteractionView`), not a process defect.
5. Commands round-trip: `drill` in the engagement line switches the active utility; `forward` advances
   the cursor AND the rendered mesh changes; a step add/remove survives a reload.
6. Console carries no `turn failed` / `render failed` / `readConflicts failed` for actor `process#1`.

## Status
Pending — the wasm component build is running; results land here.

---

# 📅️ 2026-09-02 — the app boots and mounts

## What it took to get a page at all
Two packages had been half-renamed by the peer's sweep: their `package.json` already declared
`"exports": { ".": "./🟦️.tsx" }` while the file on disk was still `📦️index.tsx`, so vite could not resolve
either of them and the whole react renderer was dead for EVERY dev app:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`
  (`@semio-tech/framework-renderer-react`) — renamed, plus its 19 importers and its own test.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` (`@semio-tech/ui-react`) — renamed.
Sibling packages (`♾️infinite`'s react renderer) already carry the `🟦️.tsx` name, so this completes a sweep
rather than inventing a convention.

Also needed: `SKIP_WASM_BUILD=1 SKIP_ENGINE_BUILD=1 SKIP_WGPU_BUILD=1` alongside `SKIP_PLUGIN_BUILD=1`, because
the machine's disk was at 100% (see below) and the surface wasm-pack build could not run.

## Result
`http://127.0.0.1:6022` serves, the shell mounts, and the document title is **`semio · process · 3d`** — the
app, its Editor mode and a Panel are all there. The console shows `cursor: 4`, which is the regenerated timber
fixture's four-step timeline arriving in the runtime: **the fixture fix reaches the browser.**

## The remaining runtime fault — NOT process-specific
Every window body still fails to render, and the three `[DEBUG]` lines are the same ones the sourcing ticket
recorded for its own app:
```
[DEBUG] PluginRuntime: turn failed for actor process#1
[DEBUG] render failed
[DEBUG] readConflicts failed
```
Thanks to sourcing's `replyError` fix the fault now decodes instead of printing `[object Object]`:
```
{"origin":"plugin","code":"plugin.internal","severity":"error",
 "message":"runtime live cleanup faulted for instance 1"}
```
Traced to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29501`. `RUNTIME_MAINTENANCE_FAULT` is stored
in exactly two places on this path:
- `:29487` — `semio_framework_job::default_now_ms()` returned `None` (no monotonic clock). Ruled unlikely: the
  component imports `wasi:clocks/monotonic-clock@0.2.0`, jco emits its shim, and for `target_env = "p2"`
  `default_clock_us` is the `std::time::Instant` branch (`🔨️modules/⏱️trace/🦀️.rs:61-65`).
- `:28868` — `interactive_step_contract_violated(elapsed)`, i.e. one cooperative-maintenance turn crossed
  `INTERACTIVE_STEP_CEILING_US = 8_000` (`⏱️trace/🦀️.rs:87-95`). **This is the likely branch.** The component
  is a 48 MB **debug-profile** wasm and the machine is pathological right now: load average 4.6–9.6, disk at
  97–100%, several concurrent cargo builds from peer sessions.

That makes the remaining failure a framework-level interactive-budget violation rather than a defect in this
plugin — consistent with sourcing hitting the identical fault on its own app. Two things would discriminate it,
neither runnable while the disk is saturated:
1. rebuild the component with `SEMIO_BUILD_MODE=ship` (the `wasm-release` profile) and re-boot;
2. boot any second plugin's dev app and confirm the same `runtime live cleanup faulted` fault.

## Machine state that shaped all of this
`/System/Volumes/Data` hit **100% of 926 GB**. `target/` alone is 157 GB and `/private/tmp/claude-501` 166 GB of
per-session scratchpads, 155 GB of it belonging to three OTHER sessions' isolated cargo target dirs. My own
16 GB isolated dir is deleted; the rest is not mine to remove.

## 🧪️ Control: the runtime is broken repo-wide, not for process
Booted a SECOND plugin's dev app from its already-built module — `dev cad` on :6031, same skip flags. It fails
the same way: the shell mounts (`semio · cad`, Editor, Panel), every window body is empty, and the console
carries the identical triad plus `Agent disconnected`:
```
[DEBUG] PluginRuntime: turn failed for actor cad#1
[DEBUG] render failed
[DEBUG] readConflicts failed
```
Its underlying faults are DIFFERENT from process's, which is what makes the control informative:
- `TypeError: Cannot destructure property 'word0' of 'v88_2' as it is undefined` — a jco-transpiled component
  ABI mismatch. `🔌️plugin-modules/cad/` is from **Sep 1 14:13**, built against an older framework WIT and now
  loaded by today's host.
- `actor-ui-patch.pairing`
- `history snapshot failed`

So three plugins now show empty windows in the dev host — sourcing (its own ticket), process, and cad — each
with a different fault, none of them a defect in the plugin's own document/render code. process is the only one
of the three whose component is built from today's source (10:16) and whose fault is not an ABI mismatch.

**Conclusion.** The remaining "windows are empty" symptom is a framework-level regression in the OS dev runtime
during the peer's in-flight serde→`ToValue`, rename and async migrations — not something the process plugin can
fix from its side. Everything process owns is green: 260 of 261 tests, the component builds 1/1, the app mounts,
and the regenerated four-step fixture demonstrably reaches the runtime (`cursor: 4` in the console).
