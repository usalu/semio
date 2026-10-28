# Exchange Model (`C/🧱️model/🔌️exchange/🐍️.py`)

Owner: EXCHANGE-MODEL agent. Stdlib only. Re-exported by the core as `step_excerpt`, `mesh`, `ids_check`. Signatures and return keys: `T/api.md` section 2 (submodule block). Self-check: `python T/exchange-selfcheck.py` (prints excerpts, verifies the invariants below, exits 1 on failure).

## What each function gives

- `step_excerpt(key)` is a complete ISO 10303-21 file fragment for **one element**: header (`FILE_SCHEMA(('IFC4X3_ADD2'))`), `DATA;` with the element, its placement, body representation, material, `Pset_<Class>Common` and (default) type and storey relations, `ENDSEC;`, `END-ISO-10303-21;`. Line `#101` is always the element (`first_id`). Everything it references is defined in the same fragment. A project, unit assignment, owner history (`$`, optional in 4.3) and the other 24 pilotis of the type relation are left out; coordinates are metres.
- `mesh(key, segments)` tessellates the piloti body: ring `segments` of vertices at base and top plus one centre vertex per cap (`2n + 2` vertices), `2n` side triangles and two fan caps (`4n` triangles).
- `ids_check(model, spec)` evaluates a minimal IDS-style requirement (applicability = IFC class, requirement = one property facet) on the model. The default requires `Pset_ColumnCommon.LoadBearing` to exist as a boolean on every `IfcColumn` and never forces `TRUE`.

## Invariants for the tests (language-agnostic)

STEP excerpt, every supported key (`A1`…`E5`, `SL-1`, `SL-R`, `W-S`, `W-E`, `W-N`, `W-W`), both languages, any `first_id`:

1. `lines[0] == "ISO-10303-21;"`, `lines[-1] == "END-ISO-10303-21;"`, `"HEADER;"` before `"DATA;"`, exactly two `ENDSEC;`, `FILE_SCHEMA(('IFC4X3_ADD2'));` present.
2. Each DATA line matches `^#(\d+)=([A-Z0-9]+)\((.*)\);$`; the numbers are unique and equal `first_id … first_id + n - 1` in line order; `ids` maps every name to its `#n` and `entities[name]` is the line index of that `#n`.
3. Every `#m` that occurs in an attribute list outside quotes is defined; parentheses and quotes balance on every line (quote doubling `''` inside strings).
4. Attribute counts (split at top-level commas): IFCCOLUMN, IFCSLAB, IFCWALL 9; IFCLOCALPLACEMENT 2; IFCAXIS2PLACEMENT3D 3; IFCAXIS2PLACEMENT2D 2; IFCCARTESIANPOINT 1; IFCDIRECTION 1; IFCPRODUCTDEFINITIONSHAPE 3; IFCSHAPEREPRESENTATION 4; IFCEXTRUDEDAREASOLID 4; IFCCIRCLEPROFILEDEF 4; IFCRECTANGLEPROFILEDEF 5; IFCGEOMETRICREPRESENTATIONCONTEXT 6; IFCMATERIAL 3; IFCMATERIALLAYER 7; IFCMATERIALLAYERSET 3; IFCRELASSOCIATESMATERIAL, IFCRELDEFINESBYPROPERTIES, IFCRELDEFINESBYTYPE, IFCRELCONTAINEDINSPATIALSTRUCTURE 6; IFCPROPERTYSINGLEVALUE 4; IFCPROPERTYSET 5; IFCCOLUMNTYPE, IFCWALLTYPE, IFCBUILDINGSTOREY 10.
5. Reals use STEP syntax `^-?\d+\.\d*(E[+-]\d+)?$` (`0.15`, `3.25`, `0.`, `1.E-05`), independent of language.
6. The element line (`#first_id`) reads `IFCCOLUMN('<GlobalId>',$,'<Name>',$,$,#placement,#shape,'<tag>',.COLUMN.)` (IFCSLAB `.FLOOR.`/`.ROOF.`, IFCWALL `.STANDARD.`); the GlobalId equals `element(key)["guid"]` (22 characters) and the tag equals `element(key)["tag"]`.
7. Spans: `0 <= start < end <= len(line)`, role in `{number, class, guid, name, tag, predefined, placement, representation, profile, depth, pset, property, value, material, reference}`; `lines[line][start:end]` is the token (string tokens without quotes): `guid` = GlobalId, `tag` = tag, `predefined` = `.COLUMN.`, `class` = `IFCCOLUMN`, `pset` = `Pset_ColumnCommon`, `property` = `LoadBearing`/`IsExternal`, `value` = `.T.`/`.F.`, `material` = material name, `depth` = extrusion depth, `profile` = circle radius (piloti) or XDim, YDim (slab, wall).
8. Values come from the model: piloti `profile` = `quantities(key)["radius"]`, `depth` = `quantities(key)["height"]`, point = `element(key)["geometry"]["axis"]` + base 0; slab depth = `quantities(key)["thickness"]`, profile = outline extent; wall `profile` = (`length`, `thickness`), `depth` = wall height; `value` tokens equal the pset values of `element(key)["psets"]`.
9. Renumbering: `step_excerpt(k, first_id=a)` and `first_id=b` have the same number of lines, the same spans (line, role), the same GlobalIds, and become identical after replacing every `#\d+` by `#`.
10. Deterministic: two calls give equal results; GlobalIds of relationship and property-set entities equal `global_id(tag + "." + suffix)` (suffixes `material`, `<Pset>`, `<Pset>.rel`, `type`, `storey`); `IFCMATERIAL` carries no GlobalId.
11. Unsupported keys (`RW-S`, `PS-1`, unknown) raise `ValueError`.
12. Size: piloti 22 DATA lines (18 with `spatial=False`), slab 22, standard wall 31, roof slab 30 (layer set of four layers); longest piloti line 89 characters.

Mesh (`segments` 3…64, default 16):

1. `vertex_count == len(vertices) == 2n + 2`, `triangle_count == len(faces) == 4n`; every face has three distinct valid indices.
2. Closed manifold: every directed edge occurs once and its reverse occurs once.
3. All ring vertices lie on the circle (centre = piloti axis, radius = `quantities(key)["radius"]`), at `z = 0` (bottom) and `z = height` (top).
4. Outward orientation and volume: the signed volume (divergence theorem) equals `n/2 · r² · sin(2π/n) · height` (`≈ π r² h` as n grows: 0.0950 at n=3, 0.1900 at n=6, 0.2239 at n=16, limit 0.2297 m³ for Ø 0.30 m, 3.25 m).
5. `kept` ids `["vertices","triangles"]`; `lost` ids `["class","guid","property","material","type"]`, each with `en` and `de`; lost items name the real class (`Class IfcColumn` / `Klasse IfcColumn`), the property (`Pset_ColumnCommon.LoadBearing`), the model's material and type names.
6. `segments` below 3, a bool or a float raise `ValueError`; non-column keys raise `ValueError`.

IDS check:

1. Default: `checked == 25`, `passed == 25`, `failed == []`, every `items[i]["value"] is True`, `applicability["entity"] == "IfcColumn"`, `requirement["property"] == "Pset_ColumnCommon.LoadBearing"`, `spec["id"] == "IDS-COLUMN-LOADBEARING"` (from `model["rules"]["ids"]`).
2. `ids_check(spec="IfcWall IsExternal exists")` checks the 4 façade walls and the solarium screen: `checked == 5`, `passed == 5`.
3. Not forced to TRUE: with `LoadBearing = False` on the type the default check still passes 25; with a `value: True` restriction it passes 0; with the property removed it fails all 25 (`failed` lists the tags); a non-boolean value fails the default `datatype`.
4. `cardinality` `prohibited` passes when the property is absent; `source="mesh"` returns `expected == 25`, `found == 0`, `checked == 0`, `passed == 0` (a mesh carries no classes or properties).
5. Texts exist in both languages: `applicability`, `requirement`, `cardinality` each have `en` and `de`.
6. Invalid spec (wrong string shape, missing entity/property, unknown cardinality or datatype, unknown source) raises `ValueError`.

## Decisions

- Piloti stay `IfcColumn` with `.COLUMN.` (our modelling choice, `T/research-ifc.md` section 15); the type relation is shown as `IFCCOLUMNTYPE` + `IFCRELDEFINESBYTYPE` for this one occurrence.
- Only the main entity is tagged `class` and `guid`, so scenes can ring `IFCCOLUMN` and the GlobalId with `token(line, role)` without catching relationship entities. The other GlobalIds are plain text.
- `depth` is the extrusion depth (the toolkit's draft role name `extrusion` maps to it); `number` marks the local `#n=` prefix (for the "local to this file" beat), `reference` marks every other `#n`.
- Slab: gross rectangular outline (`IFCRECTANGLEPROFILEDEF` positioned at the centre); the ramp void would be an `IfcOpeningElement` (`IfcRelVoidsElement`) and is not part of the excerpt. The roof slab associates the layer set (four layers) while its body depth is the concrete layer (0.20 m); the build-up above is the finish layers.
- Wall: rectangle `length × thickness` centred on the path, extruded to the roof slab soffit; the ribbon window opening (`IfcRelVoidsElement`, `IfcRelFillsElement`) is not in the excerpt.
- `FILE_DESCRIPTION` uses the `DesignTransferView` convention string from the research; the 4.3 schema does not require a specific value (research-ifc.md section 9). The time stamp is a constant so excerpts are reproducible.
- Mesh export is defined for pilotis only (slab/wall meshes would need triangulation of voids and openings).
