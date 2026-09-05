# Generation3D End-to-End Browser Verification Checklist

**Date:** 2026-09-05  
**App:** `s.procedural.generation3d` (generation3d plugin)  
**URL:** `http://localhost:6018/`  
**Objective:** Verify the app boots end-to-end and renders populated node-graph and 3D preview windows in one browser pass.

---

## 1. Node-Graph Populated-State Detection

**Empty State Selector:** `.semio-node-graph-empty` (class on `<div>` when `scene` is falsy)  
**File Reference:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1145`

### Boolean Expression (Populated ≠ Empty)

```javascript
!document.querySelector('.semio-node-graph-empty')
```

**Rationale:** When the node-graph scene is populated, the empty-state div is NOT rendered; instead, the canvas and interactive layers render. A truthy result means the graph has nodes/edges.

**Alternative (Direct Content Check):**
```javascript
document.querySelector('[data-surface-id="procedural.play.main"]')?.querySelector('[class*="surface"]') !== null
```

---

## 2. World3d Populated-State Detection

**Empty State Selector:** `.semio-world-3d-empty` (class on `<div>` when `scene` is falsy)  
**File Reference:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:5055`

### Boolean Expression (Populated ≠ Empty)

```javascript
!document.querySelector('.semio-world-3d-empty') && document.querySelector('[data-status-json]') !== null
```

**Rationale:** World3d scene is populated when:
1. The empty-state div is NOT rendered
2. A `data-status-json` attribute exists on the scene container (proves scene data is present)

### Debug Object Access (evalLen, meshesLen, instancesLen, evalHead)

**Location:** Embedded in `data-status-json` as a JSON field at path `.debug`  
**File Reference:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:69–89`

**JavaScript to Extract Debug Object:**
```javascript
const statusJsonAttr = document.querySelector('[data-status-json]')?.getAttribute('data-status-json');
const status = statusJsonAttr ? JSON.parse(statusJsonAttr) : null;
const debugObj = status?.debug;
// debugObj contains: { evalLen, meshesLen, instancesLen, evalHead }
console.log('Eval bytes:', debugObj?.evalLen);
console.log('Meshes JSON length:', debugObj?.meshesLen);
console.log('Instances JSON length:', debugObj?.instancesLen);
console.log('Eval head (first 240 chars):', debugObj?.evalHead);
```

**Proof Points:**
- `evalLen > 0` → evaluated geometry data exists
- `meshesLen > 2` (not "[]") → tessellated geometry is non-empty
- `instancesLen > 2` (not "[]") → instances (mesh placements) are non-empty

---

## 3. Node-Graph Scene Inspection (Node & Edge Count)

**Session Access:** Via the WASM canvas session (GraphWasmSession)  
**File Reference:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:709`

### JavaScript One-Liner (if session is globally exposed):

```javascript
// If the WASM session is exposed on window for inspection:
const nodeJson = window.graphSession?.selectedNodeIdsJson?.() || "[]";
const nodeIds = JSON.parse(nodeJson);
console.log(`Selected nodes: ${nodeIds.length}`);

// Alternatively, read from the component's rendered data attribute:
const surfaceData = document.querySelector('[data-surface-id="procedural.play.main"]');
const fixtureJson = surfaceData?.getAttribute('data-fixture-json');
const fixture = fixtureJson ? JSON.parse(fixtureJson) : null;
const nodeCount = fixture?.widgets?.length ?? 0;
const edgeCount = fixture?.synapses?.length ?? 0;
console.log(`Nodes: ${nodeCount}, Edges: ${edgeCount}`);
```

**Simpler Approach (Rendered Content):**
```javascript
// Count visible node elements in the React Flow diagram
const nodeElements = document.querySelectorAll('[data-surface-id="procedural.play.main"] [role="group"][data-id]');
console.log(`Visible node count: ${nodeElements.length}`);
```

---

## 4. Console Messages: Success & Known Failures

### SUCCESS Indicators

When the app boots and generation3d plugin loads, expect:

1. **Plugin descriptor fetch:** No specific log, but descriptor loads silently from plugin manifest
2. **App registry load:** Implicit — no log unless enabled in dev mode
3. **Shell boot:** Implicit success — the UI renders

**Absence of errors** is the primary success signal. Watch for the absence of these failure strings:

### FAILURE Indicators (Search Console for These Exact Strings)

| Failure Mode | Exact Console String | Cause | Recovery |
|--------------|----------------------|-------|----------|
| Descriptor invalid | `plugin.descriptor-invalid` | Manifest JSON is malformed or missing required fields | Check `http://localhost:6018/🧩️plugins/🌀️procedural/🎫️manifest.json` is valid JSON with `pluginId` and `apps` |
| Unknown app | `unknown app` | The generation3d app ID is not registered in the plugin manifest | Verify manifest declares `s.procedural.generation3d` in `apps[]` |
| Plugin load failed | `plugin.internal` | Wasm plugin failed to instantiate | Check browser console for wasm load errors; verify plugin wasm is present at the expected URL |
| Framework boot failed | `Framework OS boot failed` | OS-level boot (shell harness, document store, etc.) failed | Full boot failure; check network and shell initialization logs |
| No plugins loaded | `No plugins loaded` | Plugin list is empty or all failed to load | Verify `plugins` array passed to `FrameworkOsShell` is non-empty |
| Plugin runtime error | `PluginRuntime: turn failed for actor` | Plugin action handler raised an error | Check specific action handler implementation; test with a simpler action first |

**File Reference:** `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:*` (descriptor parsing), `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1917` (boot error), line 2674 (plugin load error)

---

## 5. Example Switcher UI Location & Interaction

**Action Declared:** `setActiveExample` (with 8-option select dropdown)  
**File References:**
- Manifest: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1172–1182`
- Handler: `set_active_example.rs:40–53`

### Where It Renders

**UI Location:** The example switcher **is NOT currently rendered in the React shell UI**. It is declared in the app's action manifest but requires a dedicated UI component to expose.

**Current State:** The `setActiveExample` action is wired and functional at the app level, but there is **no visible dropdown** in the shell (it would need to be implemented in a window chrome or panel).

### Available Examples (8 Total)

| Index | Example ID | Label (English/German) | Notes |
|-------|------------|------------------------|-------|
| 0 | `hexagonal-mushroom-column` | Hexagonal Mushroom Column / Sechseckige Pilzsäule | **Boot default** |
| 1 | `rectangle-extrude-volume` | Rectangle Extrude Volume / Rechteck-Extrusionsvolumen | |
| 2 | `sphere-cut-with-torus` | Sphere Cut With Torus / Kugel mit Torus geschnitten | |
| 3 | `box-fillet-preview` | Box Fillet Preview / Kantenrundung Vorschau | |
| 4 | `sphere-box-fuse` | Sphere Box Fuse / Kugel und Quader vereinen | |
| 5 | `face-sweep-extrude` | Face Sweep Extrude / Fläche extrudieren | |
| 6 | `rectangle-wire-preview` | Rectangle Wire Preview / Rechteck-Draht Vorschau | |
| 7 | `box-shell-preview` | Box Shell Preview / Hohlkörper Vorschau | |

### Programmatic Invocation (for verification)

Until a UI is built, test via the browser DevTools console:

```javascript
// Dispatch setActiveExample action to change the example
// (requires access to the app's dispatch function, typically via window.claudeDebug or similar)
// Example structure: { pluginId: "s.procedural", appId: "generation3d", action: "setActiveExample", args: { exampleId: "box-fillet-preview" } }
```

**File Reference for Constants:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:1–6` (example constant exports)

---

## 6. Reading Dispatch Outcomes (Action Success/Rejection)

**Goal:** Prove that `setActiveExample` or `flowEvalTick` dispatch succeeded (was not rejected with `interactive-job.missing-owned-reducer` or similar).

### Available Signals

**Direct Signal (Browser Performance API):**
```javascript
// Check if the last action dispatch was marked as a successful interactiveJob
// This requires access to the interactive-job registry, typically via:
const interactiveJobPort = window.__SEMIO_INTERACTIVE_JOB_PORT__ || window.interactiveJobPort;
// The port tracks pending jobs; when a job completes, check:
// interactiveJobPort.liveJobs() or similar (signature varies by version)
```

**Indirect Signal (State Observation):**

After dispatching `setActiveExample`, monitor:
1. **Fixture changes:** The node graph should re-tessellate (different nodes/edges appear)
2. **Preview re-renders:** The World3d scene should change (different meshes in `data-status-json`)
3. **Console silence:** No `interactive-job.missing-owned-reducer` error should appear

### Failure Indicator

If dispatch is **rejected** due to missing reducer classification, the console will print:
```
interactive-job.missing-owned-reducer
```

**File Reference:** Memory; not explicitly logged as a string. Check the interactive-job registry logs or action dispatch handler return codes.

---

## Verification Checklist (One Browser Pass)

1. [ ] **Boot:** Navigate to `http://localhost:6018/` → no console errors
2. [ ] **Plugin load:** Verify no `plugin.descriptor-invalid`, `No plugins loaded`, or `Framework OS boot failed` in console
3. [ ] **Node-graph window:** Execute `!document.querySelector('.semio-node-graph-empty')` → `true`
4. [ ] **World3d window:** Execute `!document.querySelector('.semio-world-3d-empty') && document.querySelector('[data-status-json]') !== null` → `true`
5. [ ] **Preview geometry:** Extract `statusJsonAttr` from `[data-status-json]`, parse JSON, check `debug.meshesLen > 2` and `debug.instancesLen > 2`
6. [ ] **Node & edge presence:** Verify the fixture JSON (via `data-fixture-json` attribute) contains non-empty `widgets` and `synapses` arrays
7. [ ] **Example variants exist:** Confirm examples array in manifest includes all 8 IDs (no crashes when reading constants)
8. [ ] **No interruptive errors:** Full pass with no console errors that begin with `[ERROR]`, `PluginRuntime: turn failed`, or `interactive-job.missing-owned-reducer`

---

## Summary

- **Node-graph selector:** `.semio-node-graph-empty` (absence = populated)
- **World3d selector:** `.semio-world-3d-empty` + `[data-status-json]` (both must be true = populated)
- **Debug info path:** `JSON.parse(statusJsonAttr).debug` → `{ evalLen, meshesLen, instancesLen, evalHead }`
- **Console success:** Absence of plugin errors; check for `plugin.descriptor-invalid`, `Framework OS boot failed`, `No plugins loaded`
- **Example switcher:** 8 examples wired; no UI component yet (action is functional, needs shell chrome)
- **Dispatch outcomes:** Indirect via state changes (fixtures, meshes); look for `interactive-job.missing-owned-reducer` in console on failure
