# Physical Casing Versus Nominal Analysis Dimensions

The user's requested detailed reconstruction must use the recoverable physical repository geometry, not retain a smaller nominal body because it is easier to test. The native TRIANGLES surface envelope is2.60×4.20×2.50m; its rounded body is not a wireframe/control-box artefact. Independent scan roof lengths and opposing wall planes support a physical envelope larger than2.50×4.00. Native centre offsets4.85/4.05 place physical halves2.10/1.30 exactly at the5.50m core faces. Current nominal bodies require0.10/0.05m collars to bridge invented gaps.

## Authoritative Separation

- Retain the documented nominal2.50×4.00×2.50m values for explicitly nominal dimensional/program/energy studies. Do not silently change thermal areas or teach a nominal control as an outer measured dimension.
- Add schema-first, source-bearing physical casing dimensions2.60×4.20×2.50m, derived from actual native triangle positions. Do not label this native fixture an independently surveyed historic dimension. Reference interior2.30×3.80×2.10m remains distinct.
- Physical geometry must consume the casing dimensions. Live nominal parameter changes should preserve explicit reference fabrication allowances, without duplicate hardcoded geometry rules or legacy API fallbacks. Keep control semantics explicit in renderer/consumer labels.
- Preserve world capsule centres/yaw/phases/IDs. Extend symmetrically around the existing nominal reference frame (local−.10..4.10 length,±1.30 width at defaults), or derive an equivalent physical-origin frame. A naive4.20 extrusion from the old back would shift every capsule.
- Build real casing/lining/entry/window geometry coherently. Derive actual glazing midplanes through the shared project-owned capsule_window interface. Derive door faces from physical casing boundaries; remove gap compensation rather than stack new adapters. Explicit interior dimensions govern usable lining/finished clear space.
- Physical bounds, capsule/core/bridge collisions, actual walking support and headroom must validate actual casing, not continue asserting a smaller nominal collision envelope. Retain all strict widths/tolerances and real apertures. Do not hide failures by role exclusion or shrinking the reference body.
- Renderer dimension arrows must describe the geometry they measure. Preserve clearly labelled nominal study dimensions separately from actual casing bounds. Update all affected GH/Excel source, fixtures, diagrams and bilingual narration together.

## Required Evidence

Portable physical-dimension/profile/door/window cases run red before geometry changes. NumPy/Rhino independently reproduce casing bounds, rounded profiles, centred placement and entry/window planes. Validate all six families, all140 placed physical bodies, updated33 source poses and full actual-mesh circulation/clearance. Rebuild final visual galleries and dependent native scenes only after geometry/consumer source stabilizes.

The source-backed nominal/physical distinction is a clean modeling domain separation; it must not become a compatibility alias or a silent scale wrapper. Hidden historic engineering remains explicitly uncertain. Native glazing currently supports longitudinal offset.95m while photogrammetry suggests≈.6m; retain the existing offset until the auditor resolves raw registration with direct component-relative measurements.
