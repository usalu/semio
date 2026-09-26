# Draw End-User Acceptance

The goal remains active until the editing workflows below are implemented and demonstrated in the running app. A passing pure algorithm test does not establish that an editor workflow works.

| Workflow | Required behavior | Current evidence |
| --- | --- | --- |
| Document | Create/open/save; SVG and raster import; SVG/PDF/raster export; artboard/background dimensions | Existing document and export code; runtime and roundtrip validation pending |
| Navigation | Pan, zoom, fit artwork, accurate coordinates at every zoom | Existing camera commands; runtime validation pending |
| Creation | Repeatable rectangle, ellipse, line, polygon, pen, text, image, group | Repeated layer IDs repaired; fixture test pending Rust execution |
| Selection | Click, additive/subtractive selection, marquee, actual lasso, select all/none, locked/hidden handling | Nested curve bounds, ancestor visibility/locking and frontmost picking repaired; shared fixtures added; real polygon lasso implemented with bounded coalesced samples; native gesture and browser verification pending |
| Transform | Move, resize, rotate, aspect ratio, snapping, numeric entry; cancel gesture without history changes | Numeric entry and one-leaf direct drag with ephemeral preview/release commit added; multi-selection/group drag, resize/rotate handles, snapping and runtime remain |
| Paths | Anchor/handle selection and editing, insert/delete nodes, open/close/reverse/join paths, shape conversion | Numeric anchor/handle edits, exact subdivision including arcs, delete/open/close/reverse, contour joining, segment conversion/straightening and primitive-to-path conversion added; direct handles, joining separate layers and runtime verification remain |
| Geometry | Accurate Bézier bounds; nested affine composition; Boolean operations; simplify; image trace | Exact curve/arc bounds and affine twins pass TypeScript/Three.js oracle; Boolean/trace user workflows remain unverified |
| Layers | Name, hide, lock, duplicate, delete, reorder, group/ungroup, nesting; invalid drop rejection | Inspector and atomic arrangement added; drop validation and unique creation repaired; ungroup and runtime remain |
| Appearance | Fill none/solid/gradient; stroke none/color/width/caps/joins/dashes; opacity/blending; text typography | Solid fill, enablement, stroke color/width, opacity and blend controls added; cap/join menus and dash patterns added with bounded parsing; typed gradient/stop controls and fill alpha added; typography, canvas gradient handles, large-fill scheduling and browser verification remain |
| History | Each logical edit is one undo/redo action; multi-selection edits all succeed or none do | Commands use semantic mutations and commit boundaries; runtime history validation pending |
| Interaction | Large work yields with progress/cancellation; no unbounded selection traversal | Existing retained framework owners; new selection command still uses bounded dispatch and needs a full complexity/admission audit |
| Access | English/German controls, accessible names, keyboard actions, focus behavior, customizable panels | New controls localized and labeled; actual accessibility and keyboard checks pending |
| Collaboration | Local-first event publication, coherent shared document edits, ephemeral local selection, short reconnect | Existing framework ownership used; concurrent creation identity improvements started; scenario validation pending |

## Validation Queue

1. Complete Rust compilation and execute inspector/selection/geometry/creation/drop fixtures.
2. Verify generated command publication and the actual Draw browser preview.
3. Implement semantic geometry editing, then direct-manipulation/node tools and their shared fixtures.
4. Complete appearance/layer/document workflows and keyboard access.
5. Exercise the workflows in the live app, including undo/redo, cancellation and export roundtrips.

## Path Editing Checkpoint

Path coordinate, control-handle, subdivision (line/quadratic/cubic), node deletion, contour open/close and reversal commands now have Rust/TypeScript twins and a windowed bilingual inspector surface. Shared TypeScript fixtures and independent curve/mutation oracles pass. Native and browser validation remain pending; exact arc subdivision is also implemented; direct canvas handles remain unimplemented. This does not satisfy the complete Paths or Canvas acceptance rows yet.
