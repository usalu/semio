/**
 * 🧩 Wave W2 `w2-f2-families`: the sample family set of the `office` example, merged into the snapshot by `r4-x-examples-gen.ts`. A parametric HEA 300 profile family (four lengths and one extruded I-shape)
 * is the profile of the beam type `bt-hea-300`, which carries the two steel canopy beams over the main entrance; a table family (its parameters, a cuboid top and four legs, every length a formula of the
 * parameters) is library content for the components of the next package. Every value is authored: formulas are the canonical text of the expression language; the outline, volumes and meshes are inferred.
 * `familiesFor(name, model)` returns the collections to assign; the example checks of the artifact prove that every formula names a parameter of its family.
 */
type Json = Record<string, any>;

export const zero = { x: "0 m", y: "0 m", z: "0 m" };
export const parameter = (family: string, name: string, kind: string, value: string) => [`${family}.${name}`, { family, name, kind, value }] as const;
export const solid = (family: string, name: string, shape: unknown, material: string, extra: Json = {}) => ({ family, name, shape, material, visible: "true", offset: zero, ...extra });
export const cuboid = (x: string, y: string, z: string, width: string, depth: string, height: string) => ({ Cuboid: { x, y, z, width, depth, height } });

const heaFamily = () => ({
  families: { "fam-hea-300": { name: "HEA 300", category: "Profile" } },
  family_parameters: Object.fromEntries([
    parameter("fam-hea-300", "h", "Length", "290 mm"),
    parameter("fam-hea-300", "b", "Length", "300 mm"),
    parameter("fam-hea-300", "tw", "Length", "8.5 mm"),
    parameter("fam-hea-300", "tf", "Length", "14 mm"),
  ]),
  family_solids: { "fs-hea-300-section": solid("fam-hea-300", "HEA 300 section", { Extrusion: { profile: { IShape: { width: "b", depth: "h", web: "tw", flange: "tf" } }, base: "0 m", height: "1 m" } }, "\"m-steel\"") },
});

export const tableFamily = (material: string) => ({
  families: { "fam-table": { name: "Table", category: "Furniture" } },
  family_parameters: Object.fromEntries([
    parameter("fam-table", "width", "Length", "1.6 m"),
    parameter("fam-table", "depth", "Length", "0.8 m"),
    parameter("fam-table", "height", "Length", "0.74 m"),
    parameter("fam-table", "top_thickness", "Length", "30 mm"),
    parameter("fam-table", "leg_section", "Length", "60 mm"),
    parameter("fam-table", "leg_inset", "Length", "50 mm"),
    parameter("fam-table", "leg_height", "Length", "height - top_thickness"),
    parameter("fam-table", "leg_x", "Length", "width - leg_inset - leg_section"),
    parameter("fam-table", "leg_y", "Length", "depth - leg_inset - leg_section"),
    parameter("fam-table", "seats", "Integer", "if width > 1.4 m then 6 else 4"),
  ]),
  family_solids: {
    "fs-table-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "leg_height", "width", "depth", "top_thickness"), `"${material}"`),
    "fs-table-leg-1": solid("fam-table", "Leg 1", cuboid("leg_inset", "leg_inset", "0 m", "leg_section", "leg_section", "leg_height"), `"${material}"`),
    "fs-table-leg-2": solid("fam-table", "Leg 2", cuboid("leg_x", "leg_inset", "0 m", "leg_section", "leg_section", "leg_height"), `"${material}"`),
    "fs-table-leg-3": solid("fam-table", "Leg 3", cuboid("leg_x", "leg_y", "0 m", "leg_section", "leg_section", "leg_height"), `"${material}"`),
    "fs-table-leg-4": solid("fam-table", "Leg 4", cuboid("leg_inset", "leg_y", "0 m", "leg_section", "leg_section", "leg_height"), `"${material}"`),
  },
});

export function familiesFor(name: string, model: Json): Json {
  if (name !== "office") return {};
  const material = "m-oak" in model.materials ? "m-oak" : "m-plaster";
  const hea = heaFamily();
  const table = tableFamily(material);
  const template: Json = Object.values<Json>(model.beams)[0];
  const canopy = (start: [number, number], end: [number, number], label: string) => ({ ...template, storey: "st-0", beam_type: "bt-hea-300", ...("axis" in template ? { axis: { Line: { start: { x: start[0], y: start[1] }, end: { x: end[0], y: end[1] } } } } : { start: { x: start[0], y: start[1] }, end: { x: end[0], y: end[1] } }), top_offset: -0.36, name: label });
  const result = {
    families: { ...hea.families, ...table.families },
    family_parameters: { ...hea.family_parameters, ...table.family_parameters },
    family_solids: { ...hea.family_solids, ...table.family_solids },
    beam_types: { ...model.beam_types, "bt-hea-300": { name: "HEA 300 (profile family)", profile: { Family: { family: "fam-hea-300" } }, material: "m-steel" } },
    beams: { ...model.beams, "bm-canopy-1": canopy([12, -2.4], [18, -2.4], "Canopy Beam 1"), "bm-canopy-2": canopy([12, -1.2], [18, -1.2], "Canopy Beam 2") },
  };
  return result;
}
