/**
 * 🪑️ The component and MEP vocabulary of `s.bim.model@1` (WP-10, `r11-decision-families.md` sections 4, 5, 8, 9): placed instances of families with per-instance formula overrides, and routed
 * MEP elements (ducts, pipes, cable trays) with a system. `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema, TypeScript, GraphQL and proto are generated from here.
 *
 * A component is placed by an authored plan position, rotation, mirror flag and an elevation above its storey; an optional host wall makes the inference snap it onto that wall (the authored
 * position only says on which side and where along the wall). An override is its own keyed record (key `component.name`, like a family parameter) so `(component, name)` is unique and a diff
 * names exactly the overrides it changes. A terminal is a component whose `system` is set: the connector of the family sits at the component origin and carries that system.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const componentUnitEnums = [
  { name: "MepSystem", doc: "🌀️ The building service a duct, pipe, tray or terminal belongs to: supply, return or exhaust air, domestic water, waste water, gas, power, data or lighting.", variants: ["Supply", "Return", "Exhaust", "DomesticWater", "Waste", "Gas", "Power", "Data", "Lighting"] },
];

export const componentDataEnums = [
  {
    name: "MepShape",
    doc: "🔩️ The cross-section of a routed MEP element in metres: a rectangular duct, a round pipe or a rectangular cable tray.",
    variants: [
      { name: "Duct", fields: f("width:f64, height:f64") },
      { name: "Pipe", fields: f("diameter:f64") },
      { name: "Tray", fields: f("width:f64, height:f64") },
    ],
  },
];

export const componentStructs = [
  { name: "Point3", doc: "📍️ A point in space in metres.", copy: true, fields: f("x:f64, y:f64, z:f64") },
  {
    name: "Component",
    doc: "🪑️ A placed instance of a family: furniture, equipment, casework, a fixture or a terminal. Its solids are the evaluated solids of the family under the per-instance overrides; they are inferred, never stored.",
    entity: { collection: "components", plural: "Components" },
    fields: f("storey:string, family:string, position:Point2, elevation:f64, rotation:f64, mirrored:bool, host:opt:string, system:opt:MepSystem, name:string"),
  },
  {
    name: "ComponentOverride",
    doc: "🎚️ One per-instance override of a family parameter: the formula (canonical text of the expression language) that replaces the formula of the parameter `name` for the component. Keyed by `component.name`.",
    entity: { collection: "component_overrides", plural: "ComponentOverrides" },
    fields: f("component:string, name:string, value:string"),
  },
  {
    name: "MepElement",
    doc: "🌀️ A routed MEP element: a duct, pipe or cable tray of a system along a polyline in space, its section given by the shape. Its solid, plan symbol, length and clashes are inferred.",
    entity: { collection: "mep_elements", plural: "MepElements" },
    fields: f("storey:string, system:MepSystem, shape:MepShape, path:vec:Point3, name:string"),
  },
];

export const componentFieldDocs: Record<string, string> = {
  "Component.storey": "The storey the component stands on; its elevation is measured from the elevation of this storey.",
  "Component.family": "The family the component is an instance of; its category must not be a profile.",
  "Component.position": "Plan position of the family origin in metres. With a host wall the component clings to the face of the wall on the side of this point, at the projection of this point onto the wall.",
  "Component.elevation": "Height in metres of the family origin above the elevation of the storey (negative below).",
  "Component.rotation": "Counter-clockwise turn in radians about the vertical axis through the origin, added to the orientation the host wall gives.",
  "Component.mirrored": "Whether the family is mirrored left to right (the local x axis is flipped) before it is turned.",
  "Component.host": "The wall the component is mounted on (a basin, a socket, a wall light); absent for a free-standing instance.",
  "Component.system": "The service the connector of the family carries: set for a terminal (a diffuser, a tap, a luminaire), absent otherwise.",
  "ComponentOverride.component": "The component the override belongs to.",
  "ComponentOverride.name": "The name of the family parameter whose formula the override replaces.",
  "ComponentOverride.value": "The replacing formula, in the canonical text of the expression language; it may use every parameter of the family.",
  "MepElement.path": "Centre line of the element from its start to its end: vertices in metres, the height above the elevation of the storey in z.",
  "MepElement.shape": "Cross-section across the centre line in metres: duct width and height, pipe diameter, tray width and height.",
  "MepElement.system": "The service the element carries; it decides the colour in the plan and in 3D and the group in the quantities.",
};

export const componentCollections = ["components", "component_overrides", "mep_elements"];
