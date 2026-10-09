/**
 * 🏷️ The property-set template and classification vocabulary of `s.bim.model@1` (WP-18). `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema, TypeScript, GraphQL and proto are generated from here and never drift.
 *
 * Authored: `PropertyTemplate` (a property set definition: name, the element kinds it applies to, its property definitions), `ClassificationSystem` (name, edition, an entry table code → title / parent) and the element classifications
 * `ClassificationSet` (system id → code, keyed per element like the property sets). Derived (never stored): the effective properties per element and every diagnostic about them.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const psetsUnitEnums = [
  { name: "PropertyKind", doc: "🏷️ The value type of a property definition: one of the kinds of a property value (text, real, integer, boolean, length in metres, area in square metres, volume in cubic metres, angle in radians).", variants: ["Text", "Real", "Integer", "Boolean", "Length", "Area", "Volume", "Angle"] },
  {
    name: "TemplateTarget",
    doc: "🎯️ What a property template applies to: an element kind or a type kind. A type's properties are inherited by every instance of the type unless the instance overrides them.",
    variants: ["Site", "Building", "Storey", "Wall", "CurtainWall", "Column", "Beam", "Slab", "Ceiling", "Roof", "Window", "Door", "Void", "Stair", "Ramp", "Railing", "Space", "Zone", "WallType", "SlabType", "CeilingType", "RoofType", "ColumnType", "BeamType", "WindowType", "DoorType"],
  },
];

export const psetsDataEnums: unknown[] = [];

export const psetsStructs = [
  {
    name: "PropertyDef",
    doc: "🧾️ The definition of one property of a property set template: its name and value type, an optional unit label, a default, an enumeration of allowed values, a numeric range and whether a value is required.",
    list: true,
    fields: f("name:string, kind:PropertyKind, unit:opt:string, description:opt:string, required:bool, default_value:opt:PropertyValue, allowed:vec:PropertyValue, minimum:opt:f64, maximum:opt:f64"),
  },
  {
    name: "PropertyTemplate",
    doc: "🏷️ A property set template: the property set `name` that elements (and types) of the listed kinds are expected to carry, with the definition of each of its properties. Library data: it owns no entries and no element is changed by it; the effective properties of an element are inferred.",
    entity: { collection: "property_templates", plural: "PropertyTemplates" },
    fields: f("name:string, applies_to:vec:TemplateTarget, properties:vec:PropertyDef"),
  },
  {
    name: "ClassificationItem",
    doc: "🗂️ One row of the entry table of a classification system: a code, its title and the code of its parent row (absent for a root).",
    fields: f("code:string, title:string, parent:opt:string"),
  },
  {
    name: "ClassificationSystem",
    doc: "🗂️ A classification system of the project library (for example Uniclass 2015, DIN 276 or OmniClass): its name and edition and the authored table of its entries. Library data: it owns no entries of elements.",
    entity: { collection: "classification_systems", plural: "ClassificationSystems" },
    fields: f("name:string, edition:string, source:opt:string, entries:vec:ClassificationItem"),
  },
];

export const psetsFieldDocs: Record<string, string> = {
  "PropertyDef.name": "The property name inside the property set; unique within the template.",
  "PropertyDef.kind": "The value type every value of the property must have.",
  "PropertyDef.unit": "Optional unit label printed beside the value (for example W/(m2.K)); length, area, volume and angle values are in SI units.",
  "PropertyDef.description": "Optional help text of the property.",
  "PropertyDef.required": "Whether every element the template applies to must have a value, own, inherited from its type or a default.",
  "PropertyDef.default_value": "Optional value an element has while neither it nor its type states one.",
  "PropertyDef.allowed": "The enumeration of allowed values; empty allows every value of the kind.",
  "PropertyDef.minimum": "Optional lower bound of a numeric value.",
  "PropertyDef.maximum": "Optional upper bound of a numeric value.",
  "PropertyTemplate.name": "The name of the property set the template defines; unique among the templates.",
  "PropertyTemplate.applies_to": "The element and type kinds whose elements should carry the property set; each kind at most once.",
  "PropertyTemplate.properties": "The property definitions in display order; names unique.",
  "ClassificationItem.code": "The code inside the system; unique within it.",
  "ClassificationItem.title": "The title of the entry.",
  "ClassificationItem.parent": "The code of the parent row; absent for a root; the parents form a forest.",
  "ClassificationSystem.name": "The name of the system.",
  "ClassificationSystem.edition": "The edition or version of the system.",
  "ClassificationSystem.source": "Optional publisher or location of the system.",
  "ClassificationSystem.entries": "The entry table, in display order; codes unique, every parent present.",
};
