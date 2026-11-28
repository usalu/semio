# Process Plugin Feature Inventory (2026-11-27)

These are static audits (read-only) of the hub composition, extensions, bridge, TS package and the `process3d` artifact, reconciled by the coordinator. Execution evidence lives in `coverage-matrix-2026-11-27.md`.

## Identity and contract
- The plugin id is `process` (Process/Prozess), version 0.1.0, package `semio:process` (`🌎️hub/🧩️compositions/🏭️process/🦀️.rs:41-54`).
- Domain (`✏️s/🔌️plugins/🏭️process/AGENTS.md`): it simulates the processing of components with subtractive and additive measures (cuts, drills, attachments), replayed through the brep kernel.
- The artifact is `s.process.process3d@1/*`. Its activation kind is `3d.process` and its dialect is `s.process.process3d`. Execution is isolated, and it requests `artifacts.write` (scope plugin).
- Apps:
  - Editor `#editor` (mode edit, window `process-workpiece`), crate `semio-hub-process-editor-app`.
  - Viewer `#viewer` (mode view, read-only, Noop command), crate `semio-hub-process-viewer-app`.
- Examples registered in the navbar picker: `demo` (the timber beam joinery DSL) and `concrete-forest`. The picker also offers "No example".
- The Set Active Example action additionally offers `drilled-plate`.
- Playground row: variant `process3d`, app `#editor`, react port 6022, wgpu port 6122 (hub `Cargo.toml:21-27`). It is listed in the generated `🎠️playgrounds.json`, `🔌️plugins.json` and `🖥️hosts/🦀️.rs`, and in the hub bootstrap selection.
- It consumes the topic `process.machines`.

## Extensions (declarative, contributing `process.machines`)
| extension | machines | capabilities |
|---|---|---|
| metal | chopSaw, angleGrinder, pillarDrill, magDrill, plasmaCutter, migWelder, studWelder (7) | 8 |
| wood | circularSaw, tableSaw, bandSaw, chainSaw, drillPress, cncRouter, dowelJig, screwGun (8) | 9 |
| robotic | sixAxisMill, gantryCnc, waterjet, laserCutter, assembler (5) | 7 |
| concrete | diamondSaw, wallSaw, wireSaw, coreDrill, rotaryHammer, anchorSetter, surfaceGrinder (7) | 7 |

The extensions have unit tests, a shared `catalog-json` oracle (ids and counts), and `test`/`package` targets for each `@semio-tech/process-extension-<ext>-rust`.

## Bridge
`semio-process-mutation-bridge list-mutations <artifact> <standard> <subset> [surface]` prints the runtime inventory v2 of `Process3dMutation::DESCRIPTORS`. Other coordinates are refused with exit 2. It has 3 cargo tests, and its fixture schema is validated in TS.

## TS package `@semio-tech/process-js`
The `test` target runs `bun test` on the plugin tests and then `checkRetainedRoutes`. The plugin tests are:
- `dev-route`: the launch closure, 4 machine catalogs, and the browser boot at 6022.
- `bridge-hub-fixtures`: ajv validation of the fixtures, hub app ids, and genesis.
- `typescript-package-commands`.

## Hub registration tests
- `the_registered_manifest_matches_the_shared_fixture`
- `dispatch_resolves_every_registered_app_and_refuses_unknown_ones`

Both run through `@semio-tech/process-plugin:test`.

## Editor commands (34, all declared, bridged and handled)
| group | commands | UI entry |
|---|---|---|
| Document | setSnapshot (typed bridge only), setActiveExample (palette and navbar picker), setStock (palette and catalogue), importModelFile (file picker), exportModel step/obj/stl/glb (palette), loadModelRequest (palette and catalogue) | Actions panel, navbar |
| Steps | addStep (palette, catalogue, context menu), removeSelectedStep (palette, Delete/Backspace, context menu), removeStep (trash row), moveStep (inspection), dropStep (timeline drag), updateStep (typed bridge only), setStepEnabled (eye row) | panels |
| Workshop | addWorkshopMachine, removeWorkshopMachine (workshop panel), updateWorkshopMachine (typed bridge only) | Workshop panel |
| Inspector | patchInspector | Inspection panel |
| Replay cursor | setCursor (stepper), stepCursor, stepCursorBack `[`, stepCursorForward `]` | workpiece stepper and keys |
| Engagement | engagementInput, engagementSubmit, engagementAbort (Escape); words cut/drill/attach/back/forward/all | command line |
| World | worldPointerDown, worldPointerMove, worldFaceDragEnd, placeCapability | 3D viewport, catalogue crosshair |
| View | setCamera, toggleSun, setSunAzimuth, setSunElevation, setSunIntensity | renderer and sun measures |
| Host | setContributions (`process.machines` topic) | host only |

Framework-provided actions surfaced in the Actions panel:
- Copy, Cut, Paste
- Clear Selection, Select All, Set Selection Mode, Set Granularity
- Undo, Redo, Commit Checkpoint, Create Alternative, Switch Alternative, checkoutCheckpoint, revertToCommand, history edit (13)
- Export Document, Import Document
- cancelTypedOperation

The utilities are select (default), cut, drill and attach.

## Viewer actions
copy, recordTutorial, setHistoryCommandFilter, noteShellCommand, cancelTypedOperation, hostEvent, exportArtifactDocument, importArtifactDocument.

## IO
- Export: STEP, OBJ, STL and GLB (mesh).
- Import: STEP, OBJ, STL; GLB opens display-only.
- Document codecs: DSL text, binary pack, JSON bridge, sqlite snapshot.
- Ports: `geometry:in` (STEP) and `brep:out`.
- Limits: model file 24,576 B; B-Rep pack 65,536 B.

## Reconciled flags and decisions
| # | Finding | Decision |
|---|---|---|
| F1 | setActiveExample: the option ids (`timber-beam-joinery`) disagreed with the registered example ids (`demo`), and unknown ids loaded the demo silently | **Fixed**: the id is `demo`, unknown ids are refused with a Fault, and the `plate` alias is removed |
| F2 | The dev playground lists `demo-session`, which is not a registered example | It is a generated dev-catalog value; checked by the TS worker |
| F3 | `exportModel` maps an unknown format to STEP silently | Fix: refuse it |
| F4 | `setCamera` manifest declares `up`/`zoom`, but the command ignores them | Fix: align the manifest with the command |
| F5 | Fault codes `process3d.capability.stock-limit`, `process3d.stock.display-only` and `process3d.import-model` have no localized notices | Fix: add en/de notices |
| F6 | The text-mutation decoder uses `.expect` on JSON input (`🚪️io/📝️text/🧬️mutations/🦀️.rs:134,140,152`) | Fix: return errors |
| F7 | Bridge arms default missing required args (ids `""`, setContributions `"[]"` clears the machines) | Fix: refuse invalid args (`process3d.action.invalid`) |
| F8 | Sun slider handlers are wired with `None` args | Verify that the value arrives; fix if it doesn't |
| F9 | `geometry:in` always names the payload `.step` | It is declared as STEP-only; keep it, and document that the port is STEP |
| F10 | Stale component descriptors (4 extensions, process and demonstrator hubs) | Regenerate them |
| F11 | Document swaps (stock, example, import) are outside undo | Intended by design: `LoadDocument` replaces the whole document (documented in `🗿️artifact` handler docs) |
| F12 | Bridge covers only the document lane | Intended: the inventory contract is for document mutations |
| F13 | Hub facet stubs (`setup`, `capabilities`) have doc-only claims | Low: documentation |
| F14 | Leftover `[DEBUG]` prints in framework files under active edit by other agents | Out of this ticket's ownership; the harness ignores `[DEBUG]` |
