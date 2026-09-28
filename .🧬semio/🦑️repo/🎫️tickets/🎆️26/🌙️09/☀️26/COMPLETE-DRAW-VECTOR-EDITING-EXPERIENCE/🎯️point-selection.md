# Persistent Path Point Selection

## Contract

A point is addressed by layer ID, segment index, point kind and a geometry digest. Its framework interaction target ID is `layerId:geometry:index:point`; parsing splits the last three colons, so layer IDs may contain colons or Unicode. Indices use canonical unsigned decimal within the unsigned 32-bit range shared by native and Wasm array indexing. Point kinds are anchor, control1 and control2.

The geometry digest uses the existing first-party streaming BLAKE3 implementations, with prefix ASCII `draw-points-v1`, followed by each segment in order. Segment tags are Move=0, Line=1, Quad=2, Cubic=3, Arc=4, Close=5. Coordinates use IEEE-754 binary64 little endian, normalizing negative zero. Move/Line carry endpoint x/y; Quad carries control x/y then endpoint; Cubic carries control1, control2 then endpoint; Arc carries rx, ry, rotation, one byte each for largeArc/sweep, then endpoint. Close carries no payload. Segment tags and fixed widths make the stream unambiguous. No appearance, layer transform or metadata enters the geometry digest.

This is a snapshot-bound reference, not a new persisted segment identity. Local point moves can explicitly rebind surviving selected indices to the resulting geometry. Other topology/geometry changes prune old references through normal framework topology validation. A concurrent shape edit therefore cannot silently retarget a selected index. Appearance and transform changes preserve references. Undo/history selection behavior must be tested separately.

The framework owns the point selection domain, alongside layer selection. No second private selection authority is introduced. Hidden/locked paths, including ancestor restrictions, expose no selectable points. Actual segment kinds determine available handles.

## Implementation

- Added Rust/TypeScript reference codecs, shared canonical-byte fixtures, point enumeration and visibility/lock-aware topology.
- Registered the framework-owned `points` domain alongside `strokes`. The first integrated stage deliberately declares single-point pick/replace only; it does not advertise multi-point behavior before the drag/keyboard implementation supports it.
- A retained node hit now emits a normal framework selection action without replacing layer selection. A blank node pick clears point selection. No second persistent selection store was added.
- Canvas projection distinguishes selected anchors and controls with filled markers, reading only the request-owned interaction view. Preview coordinates still come from the retained gesture owner.
- A successful node drag rebinds its selected reference to the resulting geometry. Numeric/position/translation commands rebind surviving point references on the edited path; unchanged geometry emits no history edit. Structural edits rely on topology validation to prune old references.
- Added native integration assertions for selection on pointer-down, selected control rendering, cancellation preservation, post-drag rebinding, click persistence, numeric rebinding and pruning after reversal.

## Evidence

The previous goal turn was progress: it implemented atomic multi-point translation and verified cap/join editing plus independent undo in the browser. This turn adds authoritative point-selection implementation and tests; it is not a completion claim.

TypeScript tests were run red before adding reference/topology implementations. The final Draw TypeScript target passes 118 tests across 22 files, with 18,248 assertions and the existing 44-case Ajv field-patch oracle. See `🗑️generated/tests-point-selection-final.txt` for the authoritative total. Ajv validates reference records and malformed addresses; Three.js independently checks inherited visibility/lock traversal. Rust tests compare the exact neutral byte streams with the third-party `blake3` development-only oracle.

A targeted red/green regression caught a formatter ambiguity: a malformed geometry string containing a colon could be reinterpreted as part of the layer ID. Both formatters now require the parsed fields to equal the input reference exactly. Evidence: `🗑️generated/tests-point-selection-format-red.txt` and the final TypeScript run.

Native run **71498** and component build **93519** remain live and have not been restarted. They began before the new point-selection sources and tests. Their eventual result must be inspected for coverage; run fresh native validation/build after terminal completion when necessary. No native or browser pass is claimed for point selection yet. The browser still uses the earlier component with verified stroke controls.

## Required Next Work

1. Finish native compilation and wrapper tests, resolve any failures, then rebuild/activate and demonstrate click selection, persistent filled markers, drag/rebind, cancellation and numeric editing in the browser.
2. Add multiple-point selection, modified picks, marquee, combined previews and the atomic multi-point translation command to the canvas workflow.
3. Add canvas keyboard nudge/delete and maintain the selected set across repeated edits, with each logical edit undoable once.
4. Audit topology enumeration, hashing and geometry mutation costs. The current synchronous topology contract scans path geometry; this does not satisfy the goal's large-document progress/cancellation requirement yet.
5. Verify shared edits/reconnect and history behavior, including deliberate invalidation of snapshot-bound references after other geometry changes.

The single-point stage does not complete the Paths, Selection, Interaction or Collaboration acceptance rows. The full requested editor goal remains active.

## Changed Files

- New editor interaction `🎯️points` Rust/TypeScript module, JSON schema, canonical-byte fixtures, inherited-restriction fixture and Rust unit tests.
- Editor interaction topology twins and tests.
- Editor domain declaration, request projection and window-command interaction snapshot.
- Retained pointer command selection effect and release rebinding.
- Canvas selected-point overlays.
- Semantic `editPath` local rebinding and no-op guard.
- Native editor integration tests.
- Drawing Rust development dependency on `blake3`, used only as an independent test oracle.
- This note and acceptance ledger.

## Keyboard Follow-Up

Arrow and Shift-arrow nudging is now authored for the point domain and layer domain. See [Canvas Keyboard Nudging](⌨️keyboard-nudging.md) for contracts, the 120-test TypeScript pass and outstanding native/browser validation. Node deletion and multiple-point canvas selection remain pending.
