# Preview Hover / Select / Orbit — Edit-Mode 3D Audit

Date: 2026-09-11. Scope: **read-only** audit of the generation3d **edit-mode Preview** interaction path **after** window bodies mount and eval can produce meshes. Context: Flow window already shows the Hexagonal Mushroom Column graph; wasm restaged ~14:15 local (94 138 842 B guest). Prior blocker was `unknown kind: brep.curve.polygon` on stale wasm — this audit assumes that restage lands and asks what must still be proven for hover/select/orbit.

Related: `📓️contributions-example-scope-2026-09-11.md`, `📓️guest-operator-catalogue-2026-09-11.md`, `📓️example-switch-2026-09-10.md`, `🔍️browser-probe.ts`.

---

## 1. How Preview `World3dHost` Gets Meshes, Hover, Selection, Orbit

### 1.1 End-to-end data path (plugin → host)

```
flowEvalTick (window-scoped retained route)
  → FlowEvalSession sync/tick + extension invocations (brep tessellate, math evaluate)
  → Generation3dPreviewWindowTransient.preview_eval_text  (per preview window)
  → edit_preview::render()
       preview_payload(eval, fixture, cfg, session, PreviewInteractionMarks)
       preview_selection_json(cfg, activeUtility, payload)
       world3d_scene(camera, meshes_json, instances_json, selection_json, sun)
       + domain_id: "graph", domain_granularity_id: "handle"
  → ShellHost applies BuiltNode → React World3dHost `scene` prop
  → Host div stamps probe attributes (see §1.2)
  → R3F WorldCanvas renders meshes/instances; picks dispatch back to plugin
```

**Key sources**

| Layer | File | Role |
| --- | --- | --- |
| Eval + tessellation | `✏️…/✏️editor/🦀️.rs` | `preview_payload`, `preview_tessellate_invocations`, `pending_effects` → `flowEvalTick` |
| Preview window render | `✏️…/✏️edit/…/👁️preview/🦀️.rs` | Builds `World3dScene`; threads `status_json` with `evalLen` / `meshesLen` debug |
| Window transient | `✏️…/👁️preview/🫧️transient/🦀️.rs` | Owns `preview_eval_text` published by `flowEvalTick` |
| Scene host | `🧰️…/🌐️World3dHost/🟦️.tsx` | Parses JSON, raycast pick, orbit, gumball, DOM probe attrs |
| Contributions / boot | `🧰️…/🏛️ShellHost/🟦️.tsx` | `setContributions` pack push, then `flowEvalTick`; `applyLeftoverInteractionView` on responses |

**Meshes** — `preview_payload` walks preview-enabled widgets, collects brep handles (or inline point/vector markers), resolves tessellated `MeshData` from `FlowEvalSession.preview_mesh_pack` or sync `tessellate_geometry`, dedupes by handle, emits `meshes_json` / `instances_json` arrays. Instance ids are `{widgetId}@{channel}#{index}` with `interactionId: {widgetId}@{channel}` for topology validation.

**Hover / selection (framework path)** — Domain `graph`, channel `pointer`, granularity `handle` (`GENERATION_3D_INTERACTION_*` in `✏️editor/🦀️.rs`). `PreviewInteractionMarks::from_interaction` reads `InteractionView` hover/selection and paints `hovered` / `selected` on instances; flow window uses the same marks for graph port highlight (`flow/🦀️.rs`).

**Orbit** — `WorldOrbitGated` inside `WorldCanvas` (`🌐️World3dHost/🟦️.tsx` ~5923). Right-button drag orbits (unless Alt+brush suggestion consumes the gesture). Completed orbit calls `handleCameraChange` → debounced `setCamera` dispatch to plugin (`CAMERA_SYNC_DEBOUNCE_MS`).

**Gumball** — `preview_selection_json` sets `gumballActive` when there is a selection and an active utility; `UnifiedGumball` mounts when `selection.gumballTarget` is set and transform utility is active (~2121, ~5970). Drag end commits via `world3d` gumball action chain (serialized WASM).

### 1.2 DOM bridge (`data-meshes-json` and siblings)

On the preview host root (`class="semio-world-3d-host"`):

| Attribute | Source |
| --- | --- |
| `data-meshes-json` | `scene.meshesJson` |
| `data-instances-json` | `scene.instancesJson` |
| `data-status-json` | `scene.statusJson` (computing / fault / per-widget status) |
| `data-interaction-json` | `JSON.stringify(interaction)` after **leftover merge** (§4) |
| `data-surface-id` | `node.surfaceId` (`procedural.play.preview`) |

`🔍️browser-probe.ts` `snapshotScript()` counts mesh array length from `data-meshes-json` on `.semio-world-3d-host` — this is the runtime census for “meshes exist”.

### 1.3 Pick → plugin actions (when `interactionDomainId` is set)

Preview scene sets `domain_id: "graph"`. `World3dHost` then uses the **framework** verbs (not legacy `worldPick` / `setHover`):

| Gesture | Host dispatch | Plugin route (generation3d) |
| --- | --- | --- |
| Instance hover | `interactionHover` + `{ domainId, granularity, targetId }` | Framework interaction store → `graph` / `pointer` |
| Instance click | `interactionSelect` + merge mode | Same domain; topology prunes invalid ids |
| Marquee release | `interactionSelect` with multiple `interactionId`s | `object` granularity batch |
| Empty click | `handleEmptyClick` → clear / world pick | Depends on utility mode |

Raycast: `WorldInstancesLayer` + tuned line/point thresholds (`RaycasterPickTuning`). Puzzle-specific paths (vortex, brush, fill) are inert for generation3d preview (no vortices in scene).

### 1.4 Leftover overlay (host-side, pre-guest repaint)

Until the guest republishes `selection_json` on the scene, `ShellHost` may inject a **leftover** selection/hover overlay (§4). `World3dHost` merges it:

- `mergeWorldSelectionWithLeftoverV1` — selection ids, hover id, gumball pose
- `mergeWorldInteractionWithLeftoverV1` — vortex hover / active utility (puzzle-oriented; usually noop for gen3d)

This is why `data-interaction-json` can be non-empty while `interaction` in probe snapshot is `{}` on the **guest** status object — leftover is host-local.

---

## 2. Example Switching — Do All Examples Re-Arm Eval and Preview?

### 2.1 Bundled roster

**Eight** examples are covered by native switch laws (`✏️…/🧪️tests/🔬️example-switch/🦀️.rs` `EXAMPLE_SWITCH_ORDER`):

1. `hexagonal-mushroom-column` (boot default)
2. `rectangle-extrude-volume`
3. `sphere-cut-with-torus`
4. `box-fillet-preview`
5. `sphere-box-fuse`
6. `face-sweep-extrude`
7. `rectangle-wire-preview`
8. `box-shell-preview`

Manifest also lists `demo` (demo-session); browser probe harness reports **9** combobox options (8 + demo). Contributions scoping uses **8** published example graphs (`📓️contributions-example-scope-2026-09-11.md`).

### 2.2 What `setActiveExample` does (source contract)

`🎨️set-active-example/🦀️.rs`:

- Replaces fixture via `generation3d_fixture_operations` + config camera from example.
- **`rearm_attached_previews(window_ids)`** — one `flowEvalTick` effect (route **105**) per attached `procedural-preview` window instance.

`Generation3dPreviewCommandWork` (`✏️editor/🦀️.rs` ~400): on `SetActiveExample`, extends emit with `rearm_attached_previews` from live `ViewModel` roster — **not** dependent on a later host `refresh-ui`.

`pending_effects` (~1656): when fixture has pending nodes **and** a preview window is attached, also emits `flowEvalTick` (route **104**) per preview window — boot / refresh safety net.

### 2.3 What is proven vs not

| Claim | Status |
| --- | --- |
| Each switch republishes **flow** graph node ids | **Native law** — 8/8 in `every_example_switch_publishes_…` |
| Switch arms preview **flowEvalTick** | **Native law** — `set_active_example_arms_every_attached_preview_windows_evaluation_chain` |
| Picker label + `effects: 2` in browser | **Probe** — `📓️browser-probe-harness-2026-09-10.md` (pre-bodies era) |
| Preview **meshes** update per example | **Not proven at runtime** — all probes still `meshCount: 0` |
| Flow + preview both reflect switch with eval complete | **Not proven** — eval blocked upstream of meshes |

**Conclusion:** Re-arm **logic** is wired and unit-tested for all eight examples. **Runtime** proof that each example produces `meshes > 0` and interactive preview is still open for every example.

---

## 3. `browser-probe --mode=interact` — What Each Step Clicks

Script: `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🔍️browser-probe.ts`

Default: `--steps=example,hover,select,orbit`, `--example=box-shell-preview`, `--settle=90`.

| Step | Target | Action |
| --- | --- | --- |
| **example** | Navbar `<select>` or `[role="combobox"]` | Select option matching `--example` (value or label regex); waits up to `min(settle, 20)` s |
| **hover** | Last matching preview canvas: `.semio-world-3d-host canvas`, `[data-meshes-json] canvas`, or `#framework.window.proceduralPreview canvas` | `mouse.move` to **center** of canvas bounding box; 800 ms pause |
| **select** | Same canvas locator | `click` at **fixed offset (180, 180)** px from canvas origin (not center) |
| **orbit** | Same canvas | `mouse.move` to **60% width × 45% height**; `mousedown` → `move` **(+80, +30)** over 12 steps → `mouseup` |

**Failure mode today:** If no canvas exists, steps `hover` / `select` / `orbit` throw `no preview canvas` (boot #13-pre, bodies-mesh probes). Gate exit 0 only requires shell bodies — **meshCount 0 is not fatal**.

**Not exercised by probe:** Flow window graph hover, gumball drag, shift/ctrl merge modes, context menu, utility bar, Generate mode preview, camera `setCamera` echo, or per-example loop (single `--example` only).

---

## 4. Known Leftover `InteractionView` Publisher (ShellHost / World3dHost)

### 4.1 Publisher — `ShellHost`

`🏛️ShellHost/🟦️.tsx` ~1887 `applyLeftoverInteractionView`:

1. Parses `output.interactionView` via `interactionViewFromLeftoverOutput` (`🛠️ShellHelpers/🟦️.tsx` ~408).
2. `publishLeftoverWorldSelectionV1({ ids, hoveredId, gumballActive, … })` — module-global overlay + listeners.
3. `dispatch({ type: "INTERACTION_STATE_OBSERVED", state: leftoverInteractionStateV1(...) })` for Inspection / clipboard.
4. Logs `[DEBUG] leftover InteractionView` with selected ids, hover target, gumball.

**Call sites** (after command / extension completion): ~5118 (`invokeExtension`), ~5753, ~5793, ~6003, ~8389 — any response carrying `output.interactionView`.

**Inspection side effect** (~8828): when leftover selection changes, triggers `refreshUi` scoped to Inspection panel (`leftoverInspectionRefreshScope`).

### 4.2 Consumer — `World3dHost`

`🌐️World3dHost/🟦️.tsx`:

- Subscribes to `leftoverWorldSelectionOverlayV1` via `useSyncExternalStore` (~4509–4517).
- Merges into `selection` and `interaction` before render.
- Stamps merged `interaction` on `data-interaction-json` (~5837).

**Intent (from comments):** Bridge until guest `scene.selectionJson` republishes after `interactionSelect` / `interactionHover` complete — same “leftover lane” as history patches. For generation3d, the **authoritative** path is framework `InteractionView` → `PreviewInteractionMarks` in `render_with_request_context`; leftover should be **transient** if guest keeps up.

### 4.3 Risk for this audit

Probes reading `previewHosts[].interaction` see host-merged or empty `{}` — not a substitute for proving guest hover/selection. Confirm interaction by:

- `data-interaction-json` after select step,
- `[DEBUG] interactionHover` / performInvocation lines for `interactionSelect`,
- graph window port highlight (bidirectional law in `PreviewInteractionMarks`).

---

## 5. Punchlist — Per-Example Runtime Proof (Edit Mode Preview)

**Prerequisites (once per boot, not per example):**

- [ ] Reload after wasm restage (~94 MB); no `unknown kind: brep.curve.polygon` in console / `status_json.widgetErrors`
- [ ] `setContributions` settles; `invokeExtension` to `flow-extension-brep` / `flow-extension-math` appears in console
- [ ] `data-meshes-json` length **> 0** on preview host (90 s settle or post-eval wait)
- [ ] `data-status-json` not `phase: faulted` / `flow.extension-not-contributed`

**Per example** (run probe or manual with `--example=<id>`; repeat for all eight + optional `demo`):

| Example id | Flow graph republished | `flowEvalTick` re-armed | `evalLen > 0` | `meshes > 0` | Hover (instance + graph port) | Select (gumball or highlight) | Orbit (camera moves) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hexagonal-mushroom-column` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `rectangle-extrude-volume` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `sphere-cut-with-torus` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `box-fillet-preview` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `sphere-box-fuse` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `face-sweep-extrude` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `rectangle-wire-preview` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `box-shell-preview` | native ✓ | native ✓ | ☐ | ☐ | ☐ | ☐ | ☐ |
| `demo` (9th picker) | ☐ | ☐ | ☐ | ☐ | ☐ | ☐ | ☐ |

**Suggested probe command (per example, after meshes gate passes once):**

```bash
bun 🔍️browser-probe.ts --mode=interact --steps=example,hover,select,orbit \
  --example=<id> --settle=120 --label=interact-<id>
```

Inspect `🗑️generated/probe-interact-<id>-*/step-*.snapshot.json` for `meshCount`, `previewHosts[].interaction`, and `step-orbit.png`.

**Cross-cutting checks (not per example):**

- [ ] Leftover overlay clears within one refresh after select (no stale `publishLeftoverWorldSelectionV1` ids)
- [ ] `interactionSelect` on preview instance selects port in flow graph (bidirectional hover law)
- [ ] `setCamera` fires after orbit (debounced, `[DEBUG]` or history panel)
- [ ] No `no preview canvas` on interact steps with 120 s settle post-restage

---

## 6. Current Runtime Snapshot (2026-09-11, post–14:15 restage probe)

`🗑️generated/probe-restage-wasm-1415-2026-09-11T12-22-58/settle.snapshot.json`:

- Shell **OK**: Flow + Preview windows, **3 canvases**, hex column graph visible in body text.
- **`meshCount: 0`** on both preview hosts.
- Preview `data-status-json` fault: `flow.extension-not-contributed` / `brep` (stale contributed list `[]`) on one host; flow graph status shows `extrusion-axis: computing` on the other.
- **`interaction`**: `null` / `{}` — no hover/select state to verify.

Evening status (`📓️status.md`): after example-scoped contributions pack, eval progressed (`evalLen=779`) but hit `unknown kind: brep.curve.polygon` until guest catalogue fix + restage. **Interaction audit is blocked on meshes** until that guest path is green; host-side World3dHost wiring is in place and waiting for non-empty `meshes_json`.

---

## 7. File Index (absolute)

| Path |
| --- |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` |
| `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🔍️browser-probe.ts` |
