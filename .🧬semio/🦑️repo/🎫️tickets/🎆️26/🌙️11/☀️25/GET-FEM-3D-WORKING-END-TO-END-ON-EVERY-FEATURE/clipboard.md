# Clipboard (F11): Copy / Cut / Paste in FEM 3D

## Root causes

1. Browser `UnsupportedOwner: natural-file import requires its owned declared media and no wire`: the served wasm (built 2026-11-25 12:30) predates the framework branch in `semio` `🪟️surface/✏️artifact-editor-trait/🦀️.rs` `build_reserved_tool_job` (14:53) that routes reserved `copy`/`cut`/`paste` Action inputs to `clipboard_work::EditorClipboardWork`. The old build sent them to the natural-file importer. The source is already correct; a rebuild of the component is required.
2. Native laws `sealed source requires original granted handback`: the two clipboard laws dropped a parsed `Fem3dSnapshot` (sealed `nodes` / `materials`) without retiring it, so `SealedShared::drop` asserted. The clipboard code was not at fault; the laws leaked the snapshot. Not the framework custody panic.

## Fix

- `✏️editor/📋️clipboard/🧪️tests/🦀️.rs`: snapshots held under `Retiring` (shared test guard from `🕹️interaction/🧪️tests/♻️retiring`).
- `✏️editor/📋️clipboard/🦀️.rs`: solids now copy, cut and paste (self-contained, no node references; fresh `sol` id, outline, holes and extrusion offset shifted through `FemAxis::from_world`, placement centres node and solid bounds together, unknown material falls back to the first one). Cut removes selected solids.

## Laws (`cp-law.sh`)

- copy+paste under fresh ids, no id reuse across admissions
- placement centring and empty selection
- solid paste through the extrusion axis
- cut+paste, and removing the pasted ids reverts the document (history undo itself: `undo_restores_document_after_add_node`)

## Status

All 4 laws pass natively (`cp-law.sh`, rc=0). Cut shares the stale-wasm root cause 1 (same `copy|cut|paste` branch); the session poisoning after a refused verb is the framework refusal path the selection agent changed (`plain_refusal`, `reserved-routes`), not re-verified here. Needs a component rebuild plus a browser re-probe.

## Not done

- 2D twin `◻️2d/.../📋️clipboard` does not copy regions and has no laws.
