# Draw End-User Acceptance

The goal remains active until the editing workflows below are implemented and demonstrated in the running app. A passing pure algorithm test does not establish that an editor workflow works.

| Workflow | Required behavior | Current evidence |
| --- | --- | --- |
| Document | Create/open/save; SVG and raster import; SVG/PDF/raster export; artboard/background dimensions | Existing document and export code; runtime and roundtrip validation pending |
| Navigation | Pan, zoom, fit artwork, accurate coordinates at every zoom | Shared framing request, measured viewport fitting and initial editor integration added; 15 React framing tests, 36 mounted input tests and native scene codec pass. Browser startup now fits the loaded Demo at camera (100,100,2.655), confirmed by console diagnostics and screenshot after gating both example completion and host publication; eight StrictMode startup tests pass. Artifact-level bounds now have Rust/TS twins and twelve neutral cases, including cubic/quadratic extrema and rotated image corners; the 39-test TypeScript suite passes. Viewer framing and retained local camera ownership are implemented, with independent-window and pack-restoration native tests authored; the full TypeScript suite now passes 39 tests. Six targeted native viewer tests now pass, including independent-window camera ownership/restoration and unchanged artifact bytes. The full Draw native suite passed 343/343 tests. React role switching now captures and restores complete archives before publication and reattaches saved document bindings; 23 preparation/gate/startup tests pass. Viewer retained restore hooks are implemented. Live archive replay now reaches the initializer but refuses mutation-container admission; the allocation repair is authored and awaiting native/build validation. Rollback preserves the editor and subsequent rectangle creation works. The native archive bridge roundtrip test passes with owned members and history. Successful document/history transfer and live navigation remain unverified. Native renderer/startup and explicit Fit Drawing/Selection remain pending |
| Creation | Repeatable rectangle, ellipse, line, polygon, pen, text, image, group | Layer-panel, rectangle and pen creation now have visible paint and automatic selection, verified in 334 native tests; repeated pen/polygon draft ID regression reproduced and fixed, verified in 335 native tests; browser rectangle creation, visible paint and automatic selection verified; other creation tools still need browser journeys |
| Selection | Click, additive/subtractive selection, marquee, actual lasso, select all/none, locked/hidden handling | Nested curve bounds, ancestor visibility/locking and frontmost picking repaired; shared fixtures added; real polygon lasso implemented with bounded coalesced samples; native gesture and browser verification pending |
| Transform | Move, resize, rotate, aspect ratio, snapping, numeric entry; cancel gesture without history changes | Numeric entry and direct drag with preview/release commit added; multi-selection/group drag verified in the 331-test native suite with incremental preparation, ancestor deduplication and exact undo/redo; browser verification pending; resize/rotate handles and snapping remain |
| Paths | Anchor/handle selection and editing, insert/delete nodes, open/close/reverse/join paths, shape conversion | Numeric anchor/handle edits, exact subdivision including arcs, delete/open/close/reverse, contour joining, segment conversion/straightening and primitive-to-path conversion added; direct handles, joining separate layers and runtime verification remain |
| Geometry | Accurate Bézier bounds; nested affine composition; Boolean operations; simplify; image trace | Exact curve/arc bounds and affine twins pass TypeScript/Three.js oracle; Boolean/trace user workflows remain unverified |
| Layers | Name, hide, lock, duplicate, delete, reorder, group/ungroup, nesting; invalid drop rejection | Inspector and atomic arrangement added; drop validation and unique creation repaired; rename, numeric position and lock/unlock verified in browser; ungroup and broader arrangement runtime remain |
| Appearance | Fill none/solid/gradient; stroke none/color/width/caps/joins/dashes; opacity/blending; text typography | Solid fill, enablement, stroke color/width, opacity and blend controls added; cap/join menus and dash patterns added with bounded parsing; typed gradient/stop controls and fill alpha added; semantic text content/size editing, bilingual multiline inspector controls and retained cancellation are implemented, with 46 TypeScript tests passing; native text/history and browser checks are pending. Typography layout/font controls, canvas gradient handles, large-fill scheduling and browser verification remain |
| History | Each logical edit is one undo/redo action; multi-selection edits all succeed or none do | 327-test native checkpoint verified path join/conversion and shape conversion undo/redo; multi-layer movement undo/redo now passes in the 331-test native suite; browser Undo/Redo lock transitions verified; remaining workflows pending |
| Interaction | Large work yields with progress/cancellation; no unbounded selection traversal | Existing retained framework owners; new selection command still uses bounded dispatch and needs a full complexity/admission audit |
| Access | English/German controls, accessible names, keyboard actions, focus behavior, customizable panels | New controls localized and labeled; 20 mounted EN/DE accessibility cases pass; disabled locked inspector verified in browser; keyboard workflows remain pending |
| Collaboration | Local-first event publication, coherent shared document edits, ephemeral local selection, short reconnect | Existing framework ownership used; concurrent creation identity improvements started; scenario validation pending |

## Validation Queue

1. Complete Rust compilation and execute inspector/selection/geometry/creation/drop fixtures.
2. Verify generated command publication and the actual Draw browser preview.
3. Implement semantic geometry editing, then direct-manipulation/node tools and their shared fixtures.
4. Complete appearance/layer/document workflows and keyboard access.
5. Exercise the workflows in the live app, including undo/redo, cancellation and export roundtrips.

## Path Editing Checkpoint

Path coordinate, control-handle, subdivision (line/quadratic/cubic), node deletion, contour open/close and reversal commands now have Rust/TypeScript twins and a windowed bilingual inspector surface. Shared TypeScript fixtures and independent curve/mutation oracles pass. Native and browser validation remain pending; exact arc subdivision is also implemented; direct canvas handles remain unimplemented. This does not satisfy the complete Paths or Canvas acceptance rows yet.

## Canvas Paint Checkpoint

React path construction now preserves SVG arcs, including rotated ellipse flags and multiple contours. The shared 2D raster painter uses the same geometry for fill, stroke and clips. Stroke width/dashes stay in document units across zoom, and disabled/zero-width strokes remain invisible. Eleven focused tests pass with independent Three.js SVG/path geometry oracles and neutral fixtures. Browser pixel verification and native renderer parity are still outstanding. These checks do not complete the broader Paths, Appearance or Document rows.

## Selection Topology Checkpoint

See [selection topology](🎯️selection-topology.md) for the browser reproduction, root cause, language-neutral fixtures, Rust/TypeScript twins and Three.js oracle. The TypeScript suite passes 49 tests / 1,152 assertions. Native Select All/deletion-pruning tests and the rebuilt browser behavior remain unverified while existing jobs 30810 and 43904 continue. The preview currently reports Loading plugins and its server log contains a refused trusted-catalog proxy connection. The full editing-experience goal remains open.

## Text Layout Checkpoint

[Text layout](📝️text-layout.md) records shared line handling, canvas/raster paint fixes, text-origin composition, framing and export changes. Renderer tests pass 15/15; shared 2D and Draw TypeScript targets pass. Native coverage and rebuilt text-control interaction remain pending. Font shaping and Unicode PDF embedding remain required. SVG style fidelity is addressed in the later checkpoint below; native and browser export validation remain pending.

## SVG Export Checkpoint

[SVG export](🎨️svg-export.md) records the direct typed SVG serializer and Rust/TypeScript twins. The latest Draw TypeScript run passed 54 tests / 1,197 assertions, including nonfinite geometry/paint refusal and third-party XML parsing of gradients, exact affine/path data, multiline text, image opacity and explicit absent paint. Native compilation remains pending on handles 30810 and 43904. Editable SVG roundtrip, group isolation and browser download still require validation.

The current preview recovered after a transient shared module export mismatch for `hubTransientApplyRefusalV1`; a reload restored the shell. Ongoing shared HMR then removed the action control before interaction could execute. This does not establish a successful editing journey. The source export exists; no framework change was made for this transient mismatch.


## Affine Transform Checkpoint

[Exact affine transforms](↗️affine-transforms.md) now retain shear, reflection and collapsed axes, including authored fixtures, semantic/retained mutations, digest/clone ownership and bilingual numeric controls. The TypeScript suite passes 66 tests / 1,307 assertions with Three.js and Ajv oracles. Native tests and rebuilt browser controls remain unverified; resize/rotate handle transaction integration is still outstanding. The full editing goal remains active.


## Transform Handle Checkpoint

[Transactional transform handles](🎛️transform-handles.md) implements the shared canvas selection frame and resize/rotation handles, bounded pointer preparation, absolute affine preview and one release edit. The registered TS suite passes 75 tests / 1,373 assertions. Native and browser workflow tests remain unverified. This does not yet complete the Transform, Access or Interaction acceptance rows; outstanding geometry, snapping, accessibility and large-work items are recorded in the checkpoint.

## Node Manipulation And Picking Checkpoint

[Direct path manipulation](🖱️node-manipulation.md) records two-axis semantic positioning, affine pointer math, neutral fixtures and the81-test TypeScript pass. Native command/history verification remains pending. Browser testing reproduced incorrect selection in the Demo: clicking orange selected Red Frame because the current picker accepts path bounding boxes. Precise painted-geometry hit-testing and retained canvas node gestures remain required; neither the Paths nor Selection acceptance rows are complete.
