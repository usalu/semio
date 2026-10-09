/**
 * 🧩 The parametric family vocabulary of `s.bim.model@1` (WP-23, `r11-decision-families.md`): families, their parameters and their solids. Every formula is stored as the canonical text
 * of `semio_framework_expression::print`; nothing evaluated is ever stored. `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema, TypeScript, GraphQL and
 * proto are generated from here and never drift.
 *
 * A family parameter is its own keyed record: the key of `family_parameters` is `family.name` (see `familyParameterId`), so `(family, name)` is unique, a parameter is independently
 * addressable and a diff names exactly the parameters it changes; a solid is keyed by its authored id.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const familyUnitEnums = [
  { name: "FamilyCategory", doc: "🧩 What a family is: a piece of furniture, equipment, casework, plumbing, lighting, mechanical or electrical equipment, a generic object, or a profile (a parametric cross-section a column, beam, mullion or railing uses).", variants: ["Furniture", "Equipment", "Casework", "Plumbing", "Lighting", "Mechanical", "Electrical", "Generic", "Profile"] },
  { name: "ParameterKind", doc: "🔡 The kind a family parameter holds: a length, an angle, a real or an integer number, a truth value, a text or a material (the id of a material of the project).", variants: ["Length", "Angle", "Real", "Integer", "Boolean", "Text", "Material"] },
  { name: "SolidAxis", doc: "🧭 The local axis a revolved profile turns around: the profile lies in the plane through that axis, `u` is the distance from the axis and `v` the position along it.", variants: ["X", "Y", "Z"] },
];

export const familyDataEnums = [
  {
    name: "ParametricProfile",
    doc: "▭ A closed cross-section whose dimensions are formulas: a rectangle, a circle, an I-shape or a polygon of formula points (counter-clockwise, in the local `u` right and `v` up plane).",
    variants: [
      { name: "Rectangle", fields: f("width:string, depth:string") },
      { name: "Circle", fields: f("diameter:string") },
      { name: "IShape", fields: f("width:string, depth:string, web:string, flange:string") },
      { name: "Polygon", fields: f("points:vec:ExprPoint") },
    ],
  },
  {
    name: "SolidShape",
    doc: "🧊 How a family solid is built: a profile extruded over a height, a profile revolved about an axis, a profile swept along a plan path at the elevation of the solid offset, or an axis-aligned cuboid.",
    variants: [
      { name: "Extrusion", fields: f("profile:ParametricProfile, base:string, height:string") },
      { name: "Revolution", fields: f("profile:ParametricProfile, axis:SolidAxis, angle:string") },
      { name: "Sweep", fields: f("profile:ParametricProfile, path:vec:ExprPoint") },
      { name: "Cuboid", fields: f("x:string, y:string, z:string, width:string, depth:string, height:string") },
    ],
  },
];

export const familyStructs = [
  { name: "ExprPoint", doc: "📍 A planar point whose coordinates are formulas.", fields: f("x:string, y:string") },
  { name: "ExprPoint3", doc: "📍 A point in space whose coordinates are formulas.", fields: f("x:string, y:string, z:string") },
  {
    name: "Family",
    doc: "🧩 A parametric family: a named, categorised set of parameters and solids. Its parameter values, solids and profile outline are inferred, never stored.",
    entity: { collection: "families", plural: "Families" },
    fields: f("name:string, category:FamilyCategory"),
  },
  {
    name: "FamilyParameter",
    doc: "🔢 One parameter of a family: its kind and the formula (canonical text of the expression language) that gives its value. Keyed by `family.name`.",
    entity: { collection: "family_parameters", plural: "FamilyParameters" },
    fields: f("family:string, name:string, kind:ParameterKind, value:string"),
  },
  {
    name: "FamilySolid",
    doc: "🧊 One solid of a family: its shape, the material and the visibility as formulas, and an offset of the solid in the family frame.",
    entity: { collection: "family_solids", plural: "FamilySolids" },
    fields: f("family:string, name:string, shape:SolidShape, material:string, visible:string, offset:ExprPoint3"),
  },
];

export const familyFieldDocs: Record<string, string> = {
  "FamilyParameter.family": "The family the parameter belongs to.",
  "FamilyParameter.name": "The parameter name: letters, digits and underscores, not starting with a digit; formulas refer to it by this name.",
  "FamilyParameter.value": "The formula, in the canonical text of the expression language (for example `2 * width + 40 mm`); a literal is a formula without a parameter.",
  "FamilySolid.material": "A formula of kind text: a material parameter or a quoted material id.",
  "FamilySolid.visible": "A formula of kind boolean: the solid is shown while it holds.",
  "FamilySolid.offset": "Where the origin of the solid lies in the family frame, as formulas of kind length.",
};

export const familyCollections = ["families", "family_parameters", "family_solids"];
