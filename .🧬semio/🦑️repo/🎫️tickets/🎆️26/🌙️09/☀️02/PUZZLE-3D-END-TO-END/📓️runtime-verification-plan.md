# 🧩️ Puzzle 3D End-to-End Runtime Verification Plan

This script drives the browser to validate the puzzle3d app's acceptance criteria once `bun dev:puzzle:3d` boots to `http://localhost:6013`.

## 1. Shell Boot & Window Render

**Observation:** The shell should render with the puzzle3d app loaded, showing two window panes side by side or stacked.

**Window Identifiers** (source: `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:31-32`):
- **Left/Top pane:** DOM/title shows `puzzle3d-main-top` (orthographic top view, cardinal "top" orientation)
- **Right/Bottom pane:** DOM/title shows `puzzle3d-main-perspective` (three-point perspective, free orientation)

**Expected visual layout:**
- One pane renders with an orthogonal grid from above (X-Y plane, looking down)
- One pane renders with a 3D perspective camera (three-point projection with ~50° FOV)
- Both windows are interactive, with separate camera controls and LOD/grid/vortex-display options per instance

**Failure signature (blank panes):**
- If EITHER pane renders completely blank with no geometry, no grid, no axis helpers:
  - Check browser console for `from_json_str(...).unwrap_or_else(|_| empty_fixture())` handling — this mask silent parse failures
  - Log indicates: example fixture failed to parse, fell back to empty fixture
  - Verify in Network tab: scene JSON was sent but parsing failed

**Verification step:**
```
1. Wait for page to stabilize (~3s after nav)
2. Screenshot the shell — verify two distinct viewport regions
3. Inspect DOM: query for window element ids containing "puzzle3d-main-top" and "puzzle3d-main-perspective"
4. Confirm both panes show non-blank canvas content (use canvas getImageData to detect blank)
```

---

## 2. Default Example Renders (Concrete Forest)

**Boot fixture:** `CONCRETE_FOREST_EXAMPLE_FIXTURE` (source: `✏️editor/🦀️.rs:103`)

**What should be drawn:**
- **Object count:** 1 instance (not 0, not 100+)
- **Object name:** `seed-left-001`, labeled "Hexagonal Cut Concrete Forest Left"
- **Mesh:** `/mesh/🧊️hexagonal-cut-concrete-forest-left.glb` (a hexagonal concrete structure)
- **In both panes:**
  - **Top (orthographic):** The object appears as a 2D projection from above, showing its hexagonal footprint
  - **Perspective:** The object appears with depth, showing its 3D geometry at the world origin `[0, 0, 0]`
- **Vortices:** The object has 11 vortices (connection points) visible as small spheres/indicators if "Vortex Show" is toggled on

**Failure signature (blank rendering):**
- If both panes are blank but the app is otherwise responsive (controls work, no JS errors):
  - This is the "silent empty fixture" fault: the JSON parse succeeded but yielded an empty fixture
  - Check browser console for any JSON parse warnings (none = silent failure)
  - Check Network tab for the scene JSON payload — if present, the fault is in React rendering, not wire transmission
  - Contrast with blank panes + console errors = exception during fixture load

**Verification step:**
```
1. Take screenshot of both panes with default boot
2. Count visible objects (should be exactly 1)
3. Hover over the object and read its label in the inspection panel or status bar
4. If "Vortex Show" toggle is available, toggle it on and verify 11 small indicators appear
5. Rotate the perspective pane — verify the object has 3D depth, not flat
```

---

## 3. Example Switching (setActiveExample)

**Available examples** (source: `📚️examples/{🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs:8,12`):
- **Example ID:** `concrete-forest`, **Label (EN/DE):** "Concrete Forest" / "Betonwald"
- **Example ID:** `nakagin-capsule-tower`, **Label (EN/DE):** "Nakagin Capsule Tower" / "Nakagin-Kapselturm"

**UI location** (source: `🏛️ShellHost/🟦️.tsx:6423, NavbarExampleSelect`):
- Desktop navbar, center cluster, left of mode-switcher
- Rendered as a dropdown (ComboBox or Select)
- Label: "Fixture" or app-specific terminology
- Options: English or German depending on app's active locale/terminology

**Expected behavior:**
1. Default boot shows "Concrete Forest" selected
2. Clicking the dropdown shows both options with their localized labels
3. Selecting "Nakagin Capsule Tower":
   - `setActiveExample` action is sent to the app
   - Both window panes re-render
   - Object count jumps from 1 to 180 (nakagin has 180 object instances — counted directly from the DSL, twice, independently; an earlier audit's "121" is wrong)
   - Objects are capsule-shaped units arranged vertically along a tower
4. Switching back to "Concrete Forest":
   - Object count drops back to 1
   - The hexagonal structure re-appears

**Failure signature (action rejected):**
- If clicking the example does nothing (dropdown closes but content doesn't change):
  - Check browser console for fault code `interactive-job.not-ui-safe` — indicates the action is classified `BatchOnlyPendingRewrite`, not `Migrated`
  - Check Network tab for failed action dispatch (4xx response or no response at all)
  - This is the expected blocker per ticket status: `setActiveExample` is one of the 61 unmigrated actions

**Failure signature (wrong count):**
- If nakagin renders but shows != 180 objects:
  - May indicate a different fixture was loaded
  - Check Network tab payload for example id in the scene JSON

**Verification step:**
```
1. Click the example dropdown in navbar
2. Verify both "Concrete Forest" and "Nakagin Capsule Tower" appear (check EN/DE based on locale)
3. Click "Nakagin Capsule Tower"
4. Wait 500ms for re-render
5. Screenshot both panes and count visible objects (should be ~180)
6. Inspect one object label — should show "Capsule With Balcony X" or similar (not "Hexagonal Cut...")
7. Click back to "Concrete Forest"
8. Verify object count returns to 1
```

---

## 4. Fill Tool UI & Interaction

**Fill control location** (source: `🧰️framework/.../🔬️index.test.ts` and `✏️editor/🗣️terminology/🦀️.rs:25,27`):
- Panel: Utility/tool options, grouped under "Fill" / "Füllen"
- Control type: Slider (range input)
- Control ID: `fillCount`
- Label (EN/DE): "Count" / "Anzahl"
- Range: min=1, max=9, step=1
- Default value: 3

**UI hierarchy:**
```
Group [id="fill-params", label="Fill" / "Füllen"]
  └─ Slider [id="fillCount", label="Count" / "Anzahl", value=3, min=1, max=9, step=1]
     onChange → action "setFillCount"
```

**Expected behavior:**
1. First, activate the "Fill" utility by:
   - Clicking a "Fill" tool button/icon in the window's utility toolbar, or
   - Selecting "Fill" from a mode/tool menu
2. The "Fill" panel appears in the right sidebar or bottom panel
3. The slider is visible with current value (default 3)
4. Dragging the slider to a new value (e.g., 7):
   - The `setFillCount` action is sent with args `{ fillCount: 7 }`
   - If migrated: the app's internal fill count updates
   - If NOT migrated (expected): action is rejected with `interactive-job.not-ui-safe`
5. If a `fillBuildTick` or `startFillBuild` button exists, clicking it should:
   - Trigger the fill planning process
   - Show progress indication (percentage, count, or animated object generation)
   - New objects appear in the scene as the fill advances

**User-visible state indicators for fill progress:**
- Slider value changes reflect the target fill count
- Progress indicator (if present): percentage bar, count display, or animated object appearance
- Scene objects: new objects drawn in the scene as fill completes each step

**Failure signature (control not found):**
- If the Fill panel doesn't appear or the slider isn't visible:
  - Check left/right panels for a "Fill" tab or group (screen size matters — may be in a modal)
  - Check browser console for panel rendering errors
  - Verify the "Fill" utility is actually activated (should be highlighted/selected)

**Failure signature (action not sent):**
- If dragging the slider doesn't send an action (Network tab shows no dispatch):
  - The control is rendering but not wired to an event handler
  - Check console for React warnings about missing onChange handlers

**Failure signature (action rejected):**
- If the action is sent but the app returns `interactive-job.not-ui-safe`:
  - This is the expected blocker: `setFillCount` is classified `BatchOnlyPendingRewrite`
  - Verify error appears in browser console

**Verification step:**
```
1. Click/select the "Fill" tool/utility in the window's toolbar
2. Verify a panel labeled "Fill" / "Füllen" appears with a "Count" / "Anzahl" slider
3. Verify slider current value displays (default 3)
4. Drag slider to value 7
5. Check Network tab: did setFillCount action dispatch? (look for controller="puzzle", action="setFillCount", args.fillCount=7)
6. Check browser console: did action succeed or return a fault?
   - Fault code "interactive-job.not-ui-safe" = expected blocker
   - Fault "puzzle command exceeds fixed semantic work capacity" = work extent is too large
7. If a "Start Fill" or similar button exists, click it and observe progress (or expected rejection)
```

---

## 5. Example Labels (Localization Verification)

**Terminology support** (source: `✏️editor/🗣️terminology/🦀️.rs:9-126`):
- Locale: English (`"en"`, `"en-US"`) and German (`"de"`, `"de-DE"`)
- Terminology: `native` and `reuse`
- All four combinations are supported; unsupported locale/terminology fail closed

**Label strings:**
- **Window title:** `native_en: "Puzzle 3D"`, `native_de: "Puzzle 3D"`, `reuse_en: "Aggregator"`, `reuse_de: "Aggregator"`
- **Fill label:** `native_en: "Fill"`, `native_de: "Füllen"`, `reuse_en: "Fill"`, `reuse_de: "Füllen"`
- **Fill progress label:** `native_en: "Fill progress"`, `native_de: "Füllfortschritt"`
- **Count label:** `native_en: "Count"`, `native_de: "Anzahl"`
- **Concrete Forest example:** `native_en: "Concrete Forest"`, `native_de: "Betonwald"`, `reuse_en: "Abbau Aufbau"`, `reuse_de: "Abbau Aufbau"`
- **Nakagin example:** Label from example module: `LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm")`

**Verification step (optional, if locale switching is exposed):**
```
1. If app settings expose locale/terminology selection:
   - Set locale to "de" (German)
   - Verify window title remains "Puzzle 3D"
   - Verify "Fill" panel shows "Füllen"
   - Verify slider label shows "Anzahl"
   - Verify example dropdown shows "Betonwald" and "Nakagin-Kapselturm"
2. Switch terminology from "native" to "reuse":
   - Window title should change to "Aggregator" (reuse) vs "Puzzle 3D" (native)
3. If no settings UI exists, check Network tab scene JSON or state dump for active locale/terminology
```

---

## 6. Console Error Signatures

Monitor browser console and network logs for these fault codes. Each indicates a specific migration gap.

### Fault Code: `interactive-job.not-ui-safe`

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:*` (framework plugin dispatch validation)

**Cause:** Action is classified `BatchOnlyPendingRewrite` instead of `Migrated` in the interactive-job migration

**Where it appears:** Browser console, as an action error response

**Affected actions:** `setActiveExample`, `setFillCount`, `fillBuildTick`, and ~58 others per ticket status

**Log format:**
```
{
  "kind": "ActionError",
  "code": "interactive-job.not-ui-safe",
  "message": "UI dispatch rejected [owner]:[id] with interactive-job classification BatchOnlyPendingRewrite"
}
```

**Verification:** Expected on first boot if migration is incomplete. Treat as normal until all 6 actions listed below are migrated.

### Fault Code: `typed-operation emitted a store lane absent`

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:*`

**Cause:** App's action handler emitted a mutation on a store lane (e.g., "Artifact" + "Config") that was not registered in the app's preparation factory

**Where it appears:** Browser console or server log, as a framework fault

**Likely apps:** puzzle3d (if Config or Artifact lanes are used without proper factory setup)

**Log format:**
```
{
  "kind": "Fault",
  "code": "ui-dispatch.plugin-internal",
  "message": "typed-operation emitted a store lane absent from its exact factory publication contract"
}
```

### Fault Code: `puzzle command exceeds fixed semantic work capacity`

**Source:** `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:*` (puzzle3d retained command handler)

**Cause:** A command's work extent (unit count) exceeds `PUZZLE_COMMAND_WORK_ITEMS` (likely 10,000 or similar)

**Example:** A fill command that tries to create 100,000 objects in one go

**Where it appears:** Server log or framework stderr, as a command fault

**Log format:**
```
puzzle command exceeds fixed semantic work capacity
```

**Recovery:** Reduce fill count or break the operation into smaller batches

### Fault Code: `RawWireLimit`

**Source:** `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:*` (action bus wire size validation)

**Cause:** Action payload (before decoding) exceeds the tool's `max_raw_wire_bytes` contract

**Where it appears:** Browser console, as an action dispatch error

**Log format:**
```
ToolDispatchError: tool factory '[controller]/[tool]' rejected [actual] raw bytes before decoding; maximum is [maximum]
```

**Likely scenario:** Puzzle3d scene snapshot (all objects + vortices) exceeds wire limit

### Fault Code: `runtime live cleanup faulted for instance [id]`

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:*` (plugin instance cleanup after each actor turn)

**Cause:** The app's cleanup handler (e.g., `ArtifactStore::take_returned_snapshot_read_retirement`) failed

**Known issue per ticket status:** "snapshot read retirement factory is not installed" — missing `build_artifact_store_one_item_preparation_factory` on Puzzle3dPlayApp

**Where it appears:** Server log or developer console (stderr), usually every turn

**Log format:**
```
runtime live cleanup faulted for instance [instance_id]: [detail] [[slug]] (elapsed [elapsed]us, ceiling [ceiling]us)
```

**Verification step:**
```
1. After boot, perform an interactive action (e.g., click to select, move camera)
2. Check server log/stderr for "runtime live cleanup faulted"
3. If present every turn, this is the known blocker
4. If NOT present, cleanup factory was successfully installed (good sign)
```

### Fault Code: `snapshot read retirement factory is not installed`

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14534` (artifact store retirement)

**Cause:** `ArtifactStore` was initialized without a snapshot retirement factory

**Expected on puzzle3d:** Likely causes the "runtime live cleanup faulted" fault above

**Where it appears:** Server log or stderr, nested inside the cleanup fault message

---

## 7. Checklist for Go/No-Go

| Criterion | Pass | Fail | Notes |
|-----------|------|------|-------|
| Both windows render (top + perspective) | ✓ | ✗ | If one or both are blank, check section 2 |
| Concrete Forest fixture shows 1 object | ✓ | ✗ | Should be "Hexagonal Cut Concrete Forest Left" |
| Example dropdown visible with 2 options | ✓ | ✗ | Labels: "Concrete Forest", "Nakagin Capsule Tower" (EN) or "Betonwald", "Nakagin-Kapselturm" (DE) |
| Switching to Nakagin shows ~180 objects | ✓ | ✗ | Object type should be "Capsule With Balcony" variants |
| Fill panel appears when utility selected | ✓ | ✗ | Panel label: "Fill" / "Füllen", slider label: "Count" / "Anzahl" |
| Fill slider sends `setFillCount` action | ✓ | ✗ | Check Network tab; rejection with `interactive-job.not-ui-safe` is expected if not migrated |
| No unhandled JS exceptions in console | ✓ | ✗ | Faults with expected codes (listed above) are OK |
| Camera controls work (orbit/pan/zoom) | ✓ | ✗ | Per-window, independent; test both panes |
| Scene updates reflect changes | ✓ | ✗ | Switching examples should re-render; changes should be visible within 500ms |

---

## 8. Success Criterion (Acceptance)

**The app is end-to-end ready when:**

1. ✓ Both window panes render non-blank geometry (default concrete forest fixture)
2. ✓ Example switching UI is visible and clicks send the `setActiveExample` action (rejection with `interactive-job.not-ui-safe` is acceptable if migration is pending)
3. ✓ Fill panel UI is present and the slider sends `setFillCount` actions (rejection with `interactive-job.not-ui-safe` is acceptable if migration is pending)
4. ✓ No unhandled JavaScript exceptions (faults with documented codes are OK)
5. ✓ Scene updates in both windows reflect example/fixture changes within 500ms

**Known acceptable faults (do NOT fail the acceptance test):**
- `interactive-job.not-ui-safe` on `setActiveExample`, `setFillCount`, `fillBuildTick` (expected until full migration)
- `runtime live cleanup faulted for instance 1 ... snapshot read retirement factory is not installed` (known per ticket status §46-53)

**Fail the acceptance test if:**
- Both window panes are completely blank (section 2 blank-pane signature)
- No example dropdown exists or shows only one option
- No fill panel or control after selecting fill utility
- Unhandled JS exception (error in console, not an expected fault code)
- Scene does not update when switching examples (still shows old fixture after 1 second)


---

## ⚠️ Correction applied by the coordinator

**Nakagin has 180 object instances, not 121.** Counted directly from
`📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio` as the rows between the
`objects [...] {` header (line 62) and its closing brace (line 243), and independently confirmed by the
extent-tightening work, which also measured 358 total vortex instances and 0 attraction instances.

The "121" figure originates in `📓️extent-budget-audit.md` and propagated from there. It matters here
because it is the number a runtime check would compare against, and because the same undercount made
that audit's extent overshoot figures too small (`worldRelocate` is 2.9x the cap on Nakagin, not 1.9x).

Treat any other figure inherited from that audit as a lower bound unless independently recounted.
