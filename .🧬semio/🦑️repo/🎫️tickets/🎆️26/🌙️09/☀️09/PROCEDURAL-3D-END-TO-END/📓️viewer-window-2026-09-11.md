# Generation3d viewer window — wired vs E2E surface (2026-09-11)

Read-only audit of `👁️viewer/**` under the generation3d artifact. Prior packet: `📓️viewer-2026-09-09.md`. This report compares what a user must have working end-to-end against what is actually wired in code and tests as of 2026-09-11.

---

## 0. Executive summary

| Area | Expected E2E surface | Actually wired | Gap severity |
|------|---------------------|----------------|--------------|
| Window body | Single full-pane `World3d` preview | Yes — `procedural-view-preview` / `procedural.view.preview` | Low |
| 3D geometry | Tessellated meshes from flow eval | Yes — via `preview_eval_text` transient + retained command work | **Medium** — empty on first paint until a view command runs |
| Hover / select / orbit | `graph` domain picks, camera orbit | Yes — plugin topology + `World3dHost` + `setCamera` | Low (runtime unverified on viewer boot path) |
| Seven view commands | All `Migrated`, config-only | **All real** — zero `Noop` | None |
| Eighth command | Host `setContributions` | Real — `HostOnly` lane | Low |
| Playground viewer boot | Open read-only surface like editor | **Editor-only by default** — no launch seed row sets `VITE_SEMIO_APP_ROLE=viewer` | **High** |
| Browser E2E | Viewer preview with geometry + interaction | **None** — Rust/TS contract tests only | **High** |

**Bottom line:** The viewer is no longer the Sept-9 `Noop`-only stub. All eight command rows are implemented with real handlers, retained job factories, and publication contracts. The largest E2E gaps are (1) cold-open shows an empty world until a view command drives evaluation, (2) the procedural3d playground defaults to `#editor` with no first-class viewer launch entry, and (3) no browser/runtime verification path targets `s.procedural.generation3d@1/*#viewer`.

---

## 1. Viewer / editor file map (generation3d artifact)

### 1.1 Viewer packet root

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/`

| Path | Role |
|------|------|
| `🦀️.rs` | `Generation3dViewer` trait impl, `Generation3dViewCommand` enum, retained work (`Generation3dViewCommandWork`), contributions route, `create_generation3d_viewer()` manifest |
| `🟦️.ts` | TS twin: `GENERATION3D_VIEW_ACTIONS`, `GENERATION3D_VIEW_APP_ID`, re-exports config + preview window types |
| `🎭️modes/👁️view/🦀️.rs` | Single `view` mode + full-pane layout |
| `🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` | **Preview window body** — `World3dScene` assembly, tessellation, marks, chrome measures |
| `🎭️modes/👁️view/🪟️windows/👁️preview/🟦️.ts` | Typed view-model (`Generation3dViewPreviewViewModel`, marks helpers) |
| `🎚️config/` | Persisted view state: `lod_mode`, `show_mode`, `preview_camera`, `sun_json` + four mutation leaves |
| `👥️presence/` | Shared ephemeral: `preview_camera`, `show_mode` |
| `🫧️transient/` | Local ephemeral: `preview_eval_text` (evaluated flow JSON) |
| `🎮️commands/*/` | Eight command payload modules (seven view + `setContributions`) |
| `🧪️tests/` | Unit testkit + manifest law tests |

Empty placeholder dirs (`📌️.empty.md`) remain under mode-level `🎮️commands`, `🎚️config`, etc. — intentional stubs, not wiring.

### 1.2 Editor counterpart (for comparison)

| Editor | Viewer |
|--------|--------|
| `✏️editor/🦀️.rs` — `Generation3dPlayApp`, 29+ commands, flow + preview + generate windows | `👁️viewer/🦀️.rs` — read-only, 8 commands, one preview window |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/` — node graph | **Absent** (by design, ticket 26/08/16) |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/` — editable preview + gumball | Viewer preview twin — no gumball, no `translateSelection` |
| `flowEvalTick` + preview transient on editor | Viewer uses **view-command work** to eval; no `flowEvalTick` action |

Plugin registration (`✏️s/🔌️plugins/🌀️procedural/🦀️.rs:105-106`):

```rust
.viewer::<Generation3dViewer>(create_generation3d_viewer())
```

Surface ids (`🗿️artifacts/🧊️generation3d/🦀️.rs:26`):

- Editor: `s.procedural.generation3d@1/*#editor`
- Viewer: `s.procedural.generation3d@1/*#viewer`

---

## 2. Commands — count, names, real vs Noop

### 2.1 `Generation3dViewCommand` enum (`👁️viewer/🦀️.rs:44-57`)

**Eight variants total** (via `view_commands!` macro):

| # | Action id (wire) | Rust variant | User-facing? | Implementation |
|---|------------------|--------------|--------------|----------------|
| 1 | `setShowMode` | `SetShowMode` | Yes — chrome select | **Real** — `ViewEmit::config(SetShowMode)` |
| 2 | `setLodMode` | `SetLodMode` | Yes — chrome select | **Real** — `ViewEmit::config(SetLodMode)` |
| 3 | `setCamera` | `SetCamera` | Yes — orbit/pan/zoom | **Real** — `ViewEmit::config(SetPreviewCamera)` |
| 4 | `toggleSun` | `ToggleSun` | Yes — sun chrome | **Real** — `apply_world3d_sun_action` → `SetSun` |
| 5 | `setSunAzimuth` | `SetSunAzimuth` | Yes — sun chrome | **Real** |
| 6 | `setSunElevation` | `SetSunElevation` | Yes — sun chrome | **Real** |
| 7 | `setSunIntensity` | `SetSunIntensity` | Yes — sun chrome | **Real** |
| 8 | `setContributions` | `SetContributions` | **No** — host push only | **Real** — installs flow extension pages; `ViewEmit::default()` |

**No `Noop` variant exists.** Grep over `👁️viewer/**` finds zero `Noop`/`noop` references. The Sept-9 status note ("viewer command enum is `Noop` only") is **obsolete**.

### 2.2 Retained tool factories

| Factory | Tool ids | Classification | Publication lanes |
|---------|----------|----------------|-------------------|
| `Generation3dViewBoundedCommandJobFactory` | 7 view actions | `Migrated` | Config + Presence + Transient |
| `Generation3dViewContributionsJobFactory` | `setContributions` | `Migrated` | `HostOnly` only |

`GENERATION3D_VIEW_TOOL_IDS` lists the seven user commands (`👁️viewer/🦀️.rs:70`). `setContributions` is separate (`GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS`).

Each view command's work (`Generation3dViewCommandWork`):

1. Applies config mutation via `generation3d_view_retained_reduce`
2. Runs `FlowEvalSession` to completion (progress + cancel)
3. Publishes presence (camera + show mode) + transient (`SetPreviewEval`) when eval changes

### 2.3 Manifest actions (`create_generation3d_viewer`, `👁️viewer/🦀️.rs:743-819`)

- Seven `.action_with(...)` + `.action_interactive_job(..., Migrated)` view actions
- One hidden `.command({ setContributions, in_palette: false })`
- `.window_kind_action_refs(preview::WINDOW_KIND_ID, [all seven view ids])`
- Published descriptor (`✏️s/🔌️plugins/🌀️procedural/🔣️.json:23801+`) mirrors this

### 2.4 `launch.json` entries (viewer-related)

From `.vscode/launch.json` / `.vscode/🧩️launch.seed.jsonc`:

| Name | Command | Viewer-specific? |
|------|---------|------------------|
| `🧪️generation3d-preview-window-transient` | `bun nx run workspace:generation3d-preview-window-transient` | **No** — tests **editor** preview transient contract |
| `🧪️generation3d-preview-window-transient-native` | same + `native` | **No** — editor |
| `🎮️dev🧩️generation3d⚛️react dev` | `framework-os-dev:dev-generation3d-react-dev` | **No** — boots editor role (default) |
| `🎮️serve🧩️generation3d⚛️react dev` | serve variant | **No** |
| `🔧️procedural🏙️3d` (seed) | `workspace:dev -- procedural 3d` | **No** — no `VITE_SEMIO_APP_ROLE` |
| `@semio-tech/framework-renderer-wgpu:native -- generation3d` | wgpu native | **No** |
| Build/prepare/activate generation3d rows | various `framework-os-dev:*generation3d*` | **No** |

**No launch row sets `VITE_SEMIO_APP_ROLE=viewer` or `SEMIO_APP=...#viewer` for the procedural3d playground.** Runtime verification checklist (`📓️runtime-verification-checklist-2026-09-09.md`) also targets the editor path.

### 2.5 TS action constants (`👁️viewer/🟦️.ts:18`)

```typescript
export const GENERATION3D_VIEW_ACTIONS = [
  "setShowMode", "setLodMode", "setCamera", "toggleSun",
  "setSunAzimuth", "setSunElevation", "setSunIntensity"
] as const;
```

`setContributions` is intentionally omitted (host command, not a view action).

---

## 3. Playground — viewer vs editor open path

### 3.1 React dev boot (`🧰️framework/.../🧑‍💻dev/🟦️.ts`)

```typescript
const appRole = import.meta.env.VITE_SEMIO_APP_ROLE === "viewer" ? "viewer" : "editor";
// ...
bootFrameworkOs({ ..., appId, appRole, ... });
```

- **Default role: `editor`.** Viewer opens only when `VITE_SEMIO_APP_ROLE=viewer` is set at build/serve time.
- `appId` comes from `VITE_SEMIO_APP_ID` or playground session default — still editor-biased for generation3d.

### 3.2 Shell app selection (`ShellHost/🟦️.tsx`)

When no explicit session app is pinned, manifest lookup uses `app.role === appRole`. With default `editor`, the playground loads `s.procedural.generation3d@1/*#editor` (flow + preview split), not the viewer.

### 3.3 Wgpu boot (`wgpu/🚀️browser-boot/🟦️.ts`)

```typescript
appRole: params.get("role") === "viewer" ? "viewer" : "editor"
```

Wgpu can open viewer via URL `?role=viewer` — **not wired into launch.json seed for generation3d**.

### 3.4 Examples

`generation3d_manifest_examples_are_registered_on_the_editor_surface` (`🧪️tests/🔬️surface/🦀️.rs:45-54`) — all eight examples attach to `#editor` only. Viewer has **no example dropdown** in the playground.

### 3.5 Opening viewer in practice today

| Method | Works? |
|--------|--------|
| Default `🎮️dev🧩️generation3d⚛️react dev` launch | Opens **editor** |
| Set `VITE_SEMIO_APP_ROLE=viewer` in serve env | Opens viewer app (manual) |
| Hub `open-artifact` with `role: viewer` / `#viewer` surface | Supported by framework contract; procedural viewer registered |
| wgpu `?role=viewer` | Supported in boot code; no launch row |

---

## 4. Hover / selection / camera vs World3dHost

### 4.1 Plugin side (declared surface)

| Concern | Viewer wiring | File |
|---------|---------------|------|
| Interaction domain | `graph` with `node`/`edge`/`handle` | `👁️viewer/🦀️.rs:784-801` |
| Window binding | `.window_kind_interactions(preview::WINDOW_KIND_ID, ["graph"])` | `👁️viewer/🦀️.rs:802` |
| Topology | Same widget/port/synapse projection as editor | `interaction_topology()` `👁️viewer/🦀️.rs:666-696` |
| Surface kind | `SurfaceKind::World3d` | `preview/🦀️.rs:49` |
| Scene domain | `domain_id: "graph"`, `domain_granularity_id: "handle"` | `preview/🦀️.rs:476-478` |
| Instance ids | `{widgetId}@{channel}` + `hovered`/`selected` flags | `preview/🦀️.rs:417-435` |
| Marks source | `Generation3dViewMarks::from_interaction` — framework hover + selection stores | `preview/🦀️.rs:119-123` |
| Camera in scene | `world3d_camera_json(config.preview_camera...)` | `preview/🦀️.rs:479-480` |
| Gumball | Hardcoded off (`gumballActive: false`) | `preview/🦀️.rs:451-460` |

Hover/selection are **not** duplicated in viewer transient/presence (by design — `🫧️transient/🦀️.rs:7-12`).

### 4.2 Framework host (`World3dHost/🟦️.tsx`)

| Concern | Wiring |
|---------|--------|
| Reads `scene.domainId` / `domainGranularityId` | Lines 4418-4421 |
| Hover dispatch | `interactionHover` + `world3dHoverActionArgs(domainId, granularity, id)` |
| Select dispatch | `interactionSelect` + `world3dSelectionActionArgs(...)` |
| Camera orbit → plugin | Debounced `setCamera` via `worldCameraSetCameraDispatchArgs` (line 4689) |
| Pick granularity | Uses scene's `handle` when domain declared; falls back to `WORLD3D_DEFAULT_INTERACTION_GRANULARITY` when absent |

**No leftover viewer-specific host:** viewer uses the same `World3dHost` as editor preview. The editor's flow window uses `NodeGraphHost`; viewer has no second 3D host.

### 4.3 Render path split

| Entry | Transient eval | Marks |
|-------|----------------|-------|
| `render()` (legacy) | `None` → empty meshes | Default empty |
| `render_with_request_context()` (live) | `transient.preview_eval_text` | `from_interaction(interaction)` |

Live shell uses `render_with_request_context` — correct path for hover/select + eval.

### 4.4 Cold-open geometry gap

`render_without_a_published_evaluation_paints_the_empty_world` (`preview/🧪️tests/.../🦀️.rs:172-184`):

- Without `preview_eval_text`, render emits `meshes_json: "[]"` and **zero tessellations**
- Comment in test still references `flowEvalTick` (editor mechanism); viewer has **no equivalent boot tick**

**E2E implication:** User opens viewer → sees empty world → must trigger a view command (e.g. orbit camera → `setCamera`, or change show mode) to start `Generation3dViewCommandWork` eval chain. Editor's `flowEvalTick` + `setContributions` re-arm handles this on editor open; viewer relies on shell `setContributions` + first user/chrome gesture.

---

## 5. Tests vs runtime evidence

### 5.1 Rust unit / integration (viewer-owned)

| Suite | Location | What it proves |
|-------|----------|----------------|
| Manifest + tool bijection | `👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | Role=Viewer, four-table tool id law, no artifact lane publish, all actions Migrated |
| Live dispatch | `every_viewer_action_dispatches_live_and_never_mutates_the_document` | All 7 view commands through interactive-job pipeline; document untouched |
| Window action law | `every_emitted_action_is_declared_on_the_preview_window_kind` | ShellHost `declaredAction` gate compatibility |
| Preview geometry | `👁️viewer/🎭️modes/👁️view/🧪️tests/👁️preview/🔬️unit/🦀️.rs` | Tessellation, interaction ids, marks, show mode, camera, mesh table reuse on hover |
| Per-command | `🎮️commands/*/🧪️tests/🔬️unit/🦀️.rs` | Each payload handler |
| Config leaves | `🎚️config/🧪️tests/🔬️unit/🦀️.rs` | Mutation leaf ownership on disk |

Testkit (`🧪️tests/🔬️testkit/🦀️.rs`) uses production `ViewerApp<Generation3dViewer>` — not a stub.

### 5.2 Cross-surface / descriptor tests

| Test | Location | Viewer coverage |
|------|----------|-----------------|
| `generation3d_editor_and_viewer_share_dialect` | `🧪️tests/🔬️surface/🦀️.rs` | Dialect match |
| `setContributions` receivers | `engine-contract/🟦️.ts:9446` | Viewer listed with editor |
| Window-kind action scoping | `engine-contract/🟦️.ts:9488-9489` | `setShowMode`, `setCamera` → `procedural-view-preview` |

### 5.3 What is NOT tested

| Gap | Notes |
|-----|-------|
| Browser open `s.procedural.generation3d@1/*#viewer` | No E2E Playwright/Cypress lane |
| Cold-open → geometry visible without user gesture | Explicitly tested **empty** |
| Hover/select in real `World3dHost` with viewer app | Only payload-level mark tests |
| wgpu viewer boot | wgpu lanes tested with `appRole: "editor"` |
| `generation3d-preview-window-transient` gate | Targets **editor** preview transient only |

### 5.4 Runtime evidence (ticket log)

`📓️status.md` runtime boots (#1–#12+) exercised **editor** path almost exclusively. Viewer-specific runtime verification deferred (`📓️runtime-verification-checklist-2026-09-09.md` § viewer). No confirmed boot log entry shows viewer preview with live meshes + pick.

---

## 6. E2E punchlist (viewer window)

### P0 — Must fix for viewer E2E

1. **Initial evaluation on open** — Add a boot path that publishes `preview_eval_text` without requiring user chrome/camera gesture (e.g. dedicated view bootstrap command, shell-driven initial `setLodMode`, or mirror editor's `flowEvalTick` re-arm for viewer-only sessions).
2. **Playground viewer launch row** — Add `VITE_SEMIO_APP_ROLE=viewer` (and optionally `SEMIO_APP=s.procedural.generation3d@1/*#viewer`) to `.vscode/🧩️launch.seed.jsonc` under `generation3d`, matching react + wgpu variants.
3. **Runtime verification script** — Extend `📓️runtime-verification-checklist` with viewer-specific steps: open `#viewer`, confirm meshes after contributions, orbit, pick, show-mode change.

### P1 — Should fix

4. **Viewer example or document fixture** — Either register read-only examples on viewer surface or document how hub opens a generation3d artifact in viewer role with a known fixture.
5. **Browser contract test** — Minimal `engine-contract` or Playwright step: boot with `appRole: "viewer"`, assert `procedural.view.preview` body renders non-empty `meshesJson` after settle.
6. **Confirm `setContributions` on viewer boot** — Contributions delivery landed for viewer (`📓️contributions-delivery-2026-09-10.md`); verify in viewer-role boot that paged install completes before first eval.

### P2 — Nice to have

7. **Viewer transient contract gate** — Mirror `generation3d-preview-window-transient` for viewer `🫧️transient` schema (today's gate is editor-only).
8. **`assert_viewer_never_mutates` for gen3d** — Framework helper still can't cover viewers with presence/transient (`surface/🦀️.rs:17-28`); local test already stronger.
9. **Fix stale comment** — `render_without_a_published_evaluation` test comment references `flowEvalTick` (editor); update to describe viewer's view-command chain.

### Already done (no work)

- Replace `Noop` with seven real `Migrated` view commands
- `World3d` preview surface + `graph` interaction domain
- Window chrome measures (show, LOD, sun) bound to viewer controller
- `setContributions` host route for viewer eval
- `command_from_action` bridge for all eight tool ids
- Window-kind action refs for ShellHost gate
- Hover/selection mark painting in preview payload
- Read-only gumball suppression

---

## 7. Quick reference

| Item | Value |
|------|-------|
| App id | `s.procedural.generation3d@1/*#viewer` |
| Controller id | `procedural3d-view` |
| Mode | `view` |
| Window kind | `procedural-view-preview` |
| Body key | `procedural.view.preview` |
| User view commands | **7** (all real, `Migrated`) |
| Host commands | **1** (`setContributions`) |
| `Noop` commands | **0** |
| Default playground role | **editor** |

---

*Audit method: directory walk (Glob/find), ripgrep, file reads. No files modified. No tests executed in this audit.*
