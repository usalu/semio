# D4 — Inspection panel parity report

Agent: **D4**  
Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Ownership: `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/📌️panels/🔍️inspection/`

## Summary

Extended the 5d inspection panel for compose design parity: multi-select mixed values, part anchor (Fixed/Derived), diagram `x`/`y` on parts and fasteners, all eight fastener pose parameters, and a best-effort representation/LOD select when `kindCatalogs.parts[].representations` is present.

## File touched

- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/📌️panels/🔍️inspection/🦀️component.rs`

## UI behaviour

| Selection | New / updated fields | Patch action | Args shape |
|-----------|----------------------|--------------|------------|
| Part(s) | Anchor select, flat `x`/`y`, volume origin, kind/label/text | `patchPart` | `{ "partIds": [...], "field": "<name>" }` |
| Part(s) | Representation/LOD select (if catalog has `representations`) | `patchPart` | `field`: `meshUrl`, `value`: representation `url` |
| Grip(s) | Kind, angle, radius, position, direction | `patchGrip` | `{ "gripFullIds": [...], "field": "<name>" }` |
| Fastener(s) | `gap`…`tilt` + diagram `x`/`y` | `patchFastener` | `{ "fastenerIds": [...], "field": "<name>" }` |

Mixed-value display uses `ui_inspector_mixed_text`, `ui_inspector_mixed_select`, and multi-value stepper arrays (same pattern as `🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs`).

Anchor and fastener `x`/`y` values are read via serde JSON on the live app structs so the panel stays correct once `Puzzle5dPart.anchor` and `Puzzle5dFastener.x`/`y` land in `🎛️apps/🖐️5d/🦀️component.rs` (Wave 3 integrator).

## Registration notes (out of D4 ownership)

No new mutations or manifest entries — existing `patchPart`, `patchGrip`, and `patchFastener` are reused.

**D7 / patch command owners** should extend handlers to:

1. **`patchPart`** — accept `partIds` (fallback: legacy `partId`) and apply edits to every selected part; handle `anchor` (`fixed` / `derived` camelCase serde), `meshUrl`, and existing numeric/text fields with delta broadcast like 3d `patchInspector`.
2. **`patchGrip`** — accept `gripFullIds` (fallback: `gripFullId`) for multi-edit.
3. **`patchFastener`** — accept `fastenerIds` (fallback: `fastenerId`); add `x` and `y` arms.

**D6 terminology** — optional dedicated `anchor` / `representation` inspector labels (panel currently uses `Label::data("Anchor")` and reuses `labels.lod` for the representation row).

**D7 app root** — sync play-app `Puzzle5dPart` / `Puzzle5dFastener` with artifact schema (`anchor`, fastener `x`/`y`) so anchor/diagram values persist beyond the JSON round-trip shim.

## Tests

```text
cargo test -p semio-s-plugin-puzzle inspection::
```

Result: **3 passed** (empty summary, part anchor + origin, fastener `x`/`y` steppers on nakagin example).
