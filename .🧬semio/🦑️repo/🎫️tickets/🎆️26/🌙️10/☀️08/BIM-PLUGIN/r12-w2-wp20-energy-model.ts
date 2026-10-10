/**
 * 🌡️ The energy vocabulary of `s.bim.model@1` (WP-20). `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema, TypeScript, GraphQL and proto are generated from here and never drift.
 *
 * Authored: the `SpaceConditions` of a space (occupancy, set points, ventilation, lighting and equipment power density, schedule profile reference), keyed by the id of the space it conditions (like the
 * property sets, it is not an element of its own: it lives and dies with its space), and the thermal data of the window and door types (`u_value`, `g_value`, `frame_fraction`, added to the type rows in the
 * generator). Thermal zones are the existing zones. Derived (never stored): the envelope surfaces, U-values, areas, orientations and the aggregates (the inference `energy-envelope`).
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const energyUnitEnums: unknown[] = [];

export const energyDataEnums: unknown[] = [];

export const energyStructs = [
  {
    name: "SpaceConditions",
    doc: "🌡️ The thermal conditions of one space, keyed by the id of the space: occupancy type and density, heating and cooling set points, outdoor air flow, lighting and equipment power density and the reference of the schedule profile that drives them. Every field is optional; the envelope, the areas and the U-values are inferred.",
    entity: { collection: "space_conditions", plural: "SpaceConditionsRows" },
    fields: f("occupancy:opt:string, occupancy_density:opt:f64, heating_setpoint:opt:f64, cooling_setpoint:opt:f64, ventilation_rate:opt:f64, lighting_power_density:opt:f64, equipment_power_density:opt:f64, schedule:opt:string"),
  },
];

export const windowThermalFields = "u_value:opt:f64, g_value:opt:f64, frame_fraction:opt:f64";
export const doorThermalFields = "u_value:opt:f64";

export const energyFieldDocs: Record<string, string> = {
  "SpaceConditions.occupancy": "The occupancy type of the space (for example Office, Residential or Classroom); absent leaves it unstated.",
  "SpaceConditions.occupancy_density": "Persons per square metre of floor area; absent takes the occupancy density of the zone of the space.",
  "SpaceConditions.heating_setpoint": "Heating set point in degrees Celsius; absent means the space is not heated (an unheated neighbour of a heated space is a thermal boundary).",
  "SpaceConditions.cooling_setpoint": "Cooling set point in degrees Celsius; absent means the space is not cooled.",
  "SpaceConditions.ventilation_rate": "Outdoor air flow in litres per second per square metre of floor area.",
  "SpaceConditions.lighting_power_density": "Installed lighting power in watts per square metre of floor area.",
  "SpaceConditions.equipment_power_density": "Installed equipment (plug load) power in watts per square metre of floor area.",
  "SpaceConditions.schedule": "The name of the schedule profile that drives occupancy, lighting and equipment (for example Office 08-18); the energy model resolves it.",
  "WindowType.u_value": "Thermal transmittance of the whole window (glazing and frame) in watts per square metre and kelvin; absent means the window has no thermal data.",
  "WindowType.g_value": "Total solar energy transmittance of the glazing, from 0 to 1; absent means the window has no solar data.",
  "WindowType.frame_fraction": "Share of the window opening covered by the frame, from 0 to 1; the glazed share is one minus it; absent counts the whole opening as glazing.",
  "CurtainWallType.u_value": "Thermal transmittance of the whole curtain wall (glass panels and mullions) in watts per square metre and kelvin; absent means the curtain wall has no thermal data.",
  "CurtainWallType.g_value": "Total solar energy transmittance of the glass panels, from 0 to 1; absent means the curtain wall has no solar data.",
  "CurtainWallType.frame_fraction": "Share of the curtain wall area covered by the mullions, from 0 to 1; the glazed share is one minus it; absent counts the whole area as glazing.",
  "DoorType.u_value": "Thermal transmittance of the door leaf in watts per square metre and kelvin; absent means the door has no thermal data.",
};

export const energyCollections = ["space_conditions"];
