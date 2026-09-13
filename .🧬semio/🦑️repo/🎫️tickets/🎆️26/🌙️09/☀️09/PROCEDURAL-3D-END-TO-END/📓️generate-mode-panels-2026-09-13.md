# Generate-Mode Panel Publication — 2026-09-13

Lane: the generation3d guest's Artifact / Catalogue / Inspection panel bodies in **generate** mode.
Defect: three `wgpu-ui.surface-not-published` surface faults for `framework.panel.artifact`,
`framework.panel.catalogue`, `framework.panel.inspection`
(`📓️wgpu-resident-budget-settle-2026-09-12.md` §8, `📓️audit-wgpu-journey-readiness-2026-09-13.md`
§5 lane 4).

**A procedural restage is required before the browser shows any of this.** Nothing in this lane
restaged the plugin or touched a dev server; the coordinator owns that.

---

## 1. Verdict

The guest was never the problem, and there was never any per-mode panel wiring to unify. The three
panels are declared once on the app (`✏️editor/🦀️.rs:1970-1972`), their bodies are matched in one
mode-agnostic `match` (`generation3d_render_body`, `✏️editor/🦀️.rs:244-246`), and neither
`ModeDefinition` nor either mode layout mentions a panel. Panel publication in generate mode failed
**one layer below the body-key match**, in the framework's own panel projection.

**Root cause.** `ViewModel::for_panel` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`) deliberately keeps
`focused_window_id` — that is the whole point of the field: an app-level panel that authors
per-window settings has to address the pane the user last touched
(26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1). It kept it **unconditionally**, including when the
pane is not in the host's own window roster. `VcsArtifactApp::render`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28114`) then calls

```rust
let window_config = self.window_config_store.capture(Some(view_state)).await?;   // ← faults here
```

before ANY app body key is matched, and `WindowConfigOwnerRegistry::capture`
(`🔌️plugin/🪟️window/🎚️config/🦀️.rs:527`) resolves the window as
`window_id.or(focused_window_id)` and hard-faults when the roster does not carry it:

```
Fault { origin: Framework, code: FaultCode("window-config.window-context"),
        message: "target window is absent from the exact ViewModel window instance roster" }
```

The wgpu shell boots straight into generate mode while its focused pane is still the **edit-mode**
`procedural-main`. Its own console says so, in the run this ticket already captured
(`🗑️generated/wgpu-input/wire-1/console.txt`, lines 15-51):

```
4397 wgpu-shell boot: program=procedural app=s.procedural.generation3d@1/*#editor mode=Some("generate")
4433 contributions slim view {"windowInstances":[{"id":"generation3d-generations",…},
        {"id":"generation3d-generate-form",…},{"id":"generation3d-generate-preview",…}],
        "focusedWindowId":"procedural-main","activeWindowKindId":"procedural-main"}
4530 setContributions command failed … target window is absent from the exact ViewModel window instance roster
…
4932 render begin surface=framework.panel.artifact body=procedural.play.document
4934 renderSurface surface=framework.panel.artifact turn=0 effects=0 …
4935 render leave  surface=framework.panel.artifact 2 ms          ← one turn, nothing published
…
4937 render begin surface=framework.panel.history body=framework.body.history
4953 renderSurface … turn=0 / turn=1 / turn=2 → 173 ms, resident-roots=4   ← History publishes
5126 surface fault surface=framework.panel.artifact   … wgpu-ui.surface-not-published:framework.panel.artifact
5128 surface fault surface=framework.panel.catalogue  … wgpu-ui.surface-not-published:framework.panel.catalogue
5131 surface fault surface=framework.panel.inspection … wgpu-ui.surface-not-published:framework.panel.inspection
```

Three details in that trace pin the cause exactly:

| observation | what it rules in/out |
|---|---|
| `focusedWindowId:"procedural-main"` with a three-window **generate** roster | the focused pane is not in the roster |
| `setContributions command failed … absent from the exact ViewModel window instance roster` | the very same fault, on the very same view, 400 ms earlier |
| `framework.panel.history` publishes (3 turns, 173 ms) while the three app panels leave after **one** turn in 1-2 ms | `render` returns for `FRAMEWORK_HISTORY_BODY_KEY` **before** the `capture` line; the three app panels reach it |
| all three generate WINDOWS publish normally | a window carries its own `window_id`, which `capture` resolves first |

`wgpu-ui.surface-not-published:<surface>` is then just the bridge's name for it: a guest render fault
publishes no retained document, the turn is not `more-work`, and
`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:937` throws.

**Why React shows the panels anyway.** Not host tolerance, and not an edit-mode section cache. React
runs the identical projection — `panelViewContext(request.viewState)`
(`🔌️PluginRuntime/🟦️.tsx:1758`), the exact TS twin of `for_panel` — and would fault the same way. It
does not, because `ShellHost` keeps `focusedWindowId` on a pane of the live layout, so the roster
always carries it. React was one stale focus away from the same three blank panels; the wgpu shell
merely reaches that state on every generate boot, because it seeds focus from
`app.window_kinds.first().id` regardless of the active mode
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3369, 6033, 6193`).

## 2. Fix

One projection, both languages, no per-mode duplication anywhere.

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `ViewModel::for_panel` keeps `focused_window_id` only while it names a live `window_instances` entry |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | `panelViewContext` — the exact twin — does the same |

```rust
pub fn for_panel(&self) -> Self {
    let focused_window_id = self.focused_window_id.clone().filter(|id| self.window_instances.iter().any(|window| &window.id == id));
    Self { window_id: None, active_window_kind_id: None, active_utility_id: None, focused_window_id, ..self.clone() }
}
```

A window projection is untouched: `for_window_instance` still refuses an unknown instance, and
`capture` still hard-faults for a window surface that names a pane nobody carries. Only the **panel
fallback** degrades — a panel authors against no pane instead of publishing nothing. That is the
correct reading of the field's own contract: a pane the active mode's roster does not list is not a
pane the user is looking at.

The guest needed no change. Its panel bodies were already the one shared projection the brief asked
for, with the content contract intact — Inspection publishes schema + widget count, Catalogue the
operator groups, Artifact the document tree — and the new laws now pin that.

**Side effect, not chased here:** the same fault was taking down `setContributions` on the generate
boot (console line 4530). Whether that path also flows through the panel projection is the
contributions lane's call; this fix removes the stale-focus source for every panel-scoped caller.

**For the wgpu shell lane (not mine, not touched):** the shell should seed/refresh
`focused_window_id` from the **active mode's** layout rather than `window_kinds.first()`. That is the
disagreement itself; this lane only made it survivable.

## 3. Laws

Language-agnostic fixture — mode × panel → published-body expectations, with the roster and the
focused pane of each host case:

`…/🪆️subsets/✳️any/🧫️fixtures/📌️mode-panel-publication.json`
(`semio.generation3d.mode-panel-publication` v1) — 3 panels × 4 cases: `edit`, `generate`,
`generate-stale-focus` (the wgpu boot: three generate windows, edit-mode focus) and `no-windows`.

| implementation | file | what it pins |
|---|---|---|
| Rust (plugin) | `…/✏️editor/🧪️tests/🔬️mode-panels/🦀️.rs` | every case publishes every panel with its root + markers; the body is byte-identical across all four cases; the three tabs are declared once on the app and by no mode or mode layout |
| TS twin (plugin) | `…/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts` | reads the same fixture through an independent re-implementation of the projection; asserts each case's declared focus liveness and that the live defect has its own row |
| Rust (framework) | `🛂️manifest/🧪️tests/🔬️window-view-context/🦀️.rs` + its fixture's new `panelFocus` rows | `for_panel` keeps a live focused pane and drops a stale one, without touching the roster |
| TS (React engine) | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, inside the existing `batched ui refresh request/response` (`refreshUi` sections) describe | `panelViewContext` unbinds every window-scoped field, keeps a rostered focus, drops a stale one, and drops it when there is no roster at all |

## 4. Runs

**The law fails before the fix.** With `for_panel` reverted to its pre-fix shape and nothing else
changed (`🗑️generated/generate-panels/test-prefix-revert.txt`):

```
test …::mode_panels::a_panel_publishes_the_same_body_in_every_mode ... FAILED
test …::mode_panels::every_panel_publishes_its_body_in_generate_mode_as_well_as_edit ... FAILED

case generate-stale-focus published no body for framework.panel.artifact (procedural.play.document):
Fault { origin: Framework, code: FaultCode("window-config.window-context"), severity: Error,
        message: "target window is absent from the exact ViewModel window instance roster", … }
```

`edit` and `generate` (live focus) passed in that same run — only the stale-focus case fails, which
is exactly the shape the browser shows.

**With the fix** (`🗑️generated/generate-panels/test-final2.txt`):

```
$ RUST_MIN_STACK=33554432 NX_DAEMON=false cargo test -p semio-s-artifact-procedural-generation3d \
      --features component-app-assembly --lib -- generate panel --test-threads=1
test editor::generation3d::component::mode_panels::a_panel_publishes_the_same_body_in_every_mode ... ok
test editor::generation3d::component::mode_panels::every_panel_publishes_its_body_in_generate_mode_as_well_as_edit ... ok
test editor::generation3d::component::mode_panels::the_app_declares_one_shared_panel_per_framework_tab_and_no_mode_declares_its_own ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 373 filtered out; finished in 0.73s
```

Zero warnings name any file this lane added.

**React vitest** (`🗑️generated/generate-panels/vitest-2.txt`), the `refreshUi` sections suite the new
`panelViewContext` laws live in:

```
$ SEMIO_TEST_LEVEL=long node node_modules/vitest/vitest.mjs run --config ../../🧪️tests/🎚️config/🟦️.ts \
      ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --reporter=verbose \
      --testNamePattern="panelViewContext|buildUiRefreshRequest"
✓ panelViewContext unbinds a panel from every window-scoped field
✓ panelViewContext keeps a focused pane the roster carries, so per-window panel controls still address it
✓ panelViewContext drops a focused pane the active mode's roster does not carry, so a mode-switch race cannot unpublish every app panel
✓ panelViewContext drops a focused pane when the host carries no roster at all
Test Files  1 passed (1)
     Tests  14 passed | 585 skipped (599)
```

**Plugin TS twin:**

```
$ bun …/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts
generation3d mode-panel publication panels=3 cases=4 staleFocusCases=generate-stale-focus,no-windows
```

**Framework manifest law** (`🗑️generated/generate-panels/test-manifest3.txt`), once the peer's
`semio-framework-pack` refactor in §5 went green:

```
$ RUST_MIN_STACK=33554432 NX_DAEMON=false cargo test -p semio-framework --lib -- window_view_context --test-threads=1
test manifest::window_view_context_tests::window_view_context_uses_the_addressed_instance ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 235 filtered out
```

The plugin law was re-run afterwards on that same settled tree
(`🗑️generated/generate-panels/test-final3.txt`): `15 passed; 0 failed`.

## 5. Observed, not mine

| red | peer work in flight |
|---|---|
| `semio-framework-os-infinite` — 4 errors at `💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:504-545`: a duplicated `#[derive(Clone, Debug)]` on `WorldCatalogueDropPreviewRecord` and a lost `Default`/`Clone` on `WorldBrushPreviewRecord`. File mtime 5 minutes before the failure. Waited; it cleared on its own. Not touched. |
| `semio_framework::Viewport2d` / `semio_framework_plugin::Viewport2d` no longer exist — the move to `semio_framework_os_kernel::Viewport2d` (re-exported at `💻️os/📦️packages/🦀️rust/🦀️.rs:344`) that `📓️wgpu-resident-budget-settle-2026-09-12.md` §10 already recorded reaching wasm32. It had now reached this plugin and blocked its whole test binary. The four consumers were moved FORWARD onto the new path (`✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`, `✏️editor/🦀️.rs:192`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:854`). **Nothing was reverted** — the peer's move was completed, not undone. |
| `semio-framework-pack` went red mid-session: `DeflateRetainedCursor::try_new` grew a third `maximum_allocation_bytes: usize` argument and `close` became `close_step(maximum_items, maximum_bytes)` in `📡️replication/⚙️codec/🦀️.rs`, while `🎒️pack/📐️format/🦀️.rs:1551,1666,1717` still called the old shapes — blocking `cargo test -p semio-framework`. Polled across 12 attempts over ~12 minutes (error count 3 → 2 → green); the peer landed it and both framework runs in §4 were taken afterwards. Not this lane's files; not touched. |

Nothing was reverted and nothing was fought. Shared cargo build dir throughout,
`RUST_MIN_STACK=33554432 NX_DAEMON=false`, every command in the foreground.

## 6. Files

**New**

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/📌️mode-panel-publication.json`
- `…/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🦀️.rs`
- `…/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️generate-mode-panels-2026-09-13.md`

**Changed**

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `ViewModel::for_panel` drops a focused pane the roster does not carry; field docstring sharpened
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — `panelViewContext`, the twin, same rule; field docstring sharpened
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🦀️.rs` + `🧫️fixtures/🔬️window-view-context/🔣️.json` — `panelFocus` rows and their law
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — four `panelViewContext` laws in the `refreshUi` sections suite
- `…/✏️editor/🦀️.rs` — registers `mod mode_panels`; `Viewport2d` moved forward (peer's move)
- `…/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`, `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`, `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `Viewport2d` moved forward (peer's move)

**Generated (delete with the ticket)**

- `🗑️generated/generate-panels/{compile-1,compile-2,test-1,test-prefix-revert,test-final,test-final2,test-final3,test-manifest,test-manifest2,test-manifest3,vitest-1,vitest-2}.txt`
