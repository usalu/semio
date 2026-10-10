/**
 * 🌡️ WP-20 `w2-wp20-energy`: the thermal data of the `house` and `office` examples, merged into the snapshot source by `r4-x-examples-gen.ts`: the U-value, g-value and frame fraction of every window type, the U-value of every
 * door type and the conditions of the spaces (occupancy, set points, ventilation, lighting and equipment power density, schedule profile). The zones of the examples are the thermal zones. The envelope, areas, orientations and
 * U-values of the walls, slabs and roofs are inferred from the layer stacks and never authored. `energyFor(name, model)` edits `model` in place and refuses a condition that names no space or breaks the limits of the leaves.
 *
 * Window and door values after the usual EnEV/GEG reference data: triple glazing Uw 0.8 to 1.1, double glazing Uw 1.3 to 1.4, timber doors 1.3 to 1.8.
 */
type Json = Record<string, any>;

const WINDOWS: Record<string, [number, number, number]> = {
  "wn-cellar": [1.4, 0.55, 0.3],
  "wn-wc": [1.1, 0.5, 0.25],
  "wn-casement": [0.95, 0.5, 0.25],
  "wn-picture": [0.8, 0.5, 0.15],
  "wn-bay": [1.0, 0.5, 0.25],
  "wn-ribbon": [1.3, 0.45, 0.2],
};

const DOORS: Record<string, number> = {
  "dr-entry": 1.3,
  "dr-int-90": 2.0,
  "dr-int-80": 2.0,
  "dr-cellar": 1.8,
  "dr-patio": 1.1,
  "dr-glass": 1.4,
  "dr-core": 1.8,
};

const conditions = (occupancy: string, density: number, heating: number | undefined, cooling: number | undefined, ventilation: number, lighting: number, equipment: number, schedule: string) => ({
  occupancy,
  occupancy_density: density,
  ...(heating === undefined ? {} : { heating_setpoint: heating }),
  ...(cooling === undefined ? {} : { cooling_setpoint: cooling }),
  ventilation_rate: ventilation,
  lighting_power_density: lighting,
  equipment_power_density: equipment,
  schedule,
});

const unheated = (occupancy: string, lighting: number, schedule: string) => ({ occupancy, occupancy_density: 0.01, lighting_power_density: lighting, schedule });

const HOUSE: Record<string, Json> = {
  "sp-b1": unheated("Circulation", 2, "Residential day"),
  "sp-b2": unheated("Storage", 3, "Residential day"),
  "sp-g1": conditions("Circulation", 0.02, 18, undefined, 0.2, 4, 1, "Residential day"),
  "sp-g2": conditions("Sanitary", 0.02, 20, undefined, 0.5, 5, 1, "Residential day"),
  "sp-g3": conditions("Circulation", 0.02, 18, undefined, 0.2, 4, 1, "Residential day"),
  "sp-g4": conditions("Living", 0.05, 21, 26, 0.3, 6, 4, "Residential day"),
  "sp-g5": conditions("Kitchen", 0.05, 21, 26, 0.4, 6, 8, "Residential day"),
  "sp-u1": conditions("Circulation", 0.02, 18, undefined, 0.2, 4, 1, "Residential night"),
  "sp-u2": conditions("Sanitary", 0.02, 24, undefined, 0.5, 5, 2, "Residential night"),
  "sp-u3": conditions("Sleeping", 0.03, 18, 26, 0.3, 4, 2, "Residential night"),
  "sp-u4": conditions("Sleeping", 0.03, 20, 26, 0.3, 4, 2, "Residential night"),
};

const office = (level: number): Record<string, Json> => ({
  [`sp-${level}-1`]: conditions("Office", 0.1, 21, 25, 1.5, 8, 12, "Office 08-18"),
  [`sp-${level}-2`]: conditions("Office", 0.1, 21, 25, 1.5, 8, 12, "Office 08-18"),
  [`sp-${level}-3`]: conditions("Circulation", 0.02, 18, undefined, 0.3, 4, 1, "Office 08-18"),
  [`sp-${level}-4`]: conditions("Circulation", 0.02, 18, undefined, 0.3, 4, 1, "Office 08-18"),
  [`sp-${level}-5`]: conditions("Circulation", 0.02, 18, undefined, 0.3, 4, 1, "Office 08-18"),
  [`sp-${level}-6`]: unheated("Escape stair", 2, "Office 08-18"),
  [`sp-${level}-7`]: unheated("Escape stair", 2, "Office 08-18"),
});

const check = (id: string, row: Json) => {
  const heating = row.heating_setpoint;
  const cooling = row.cooling_setpoint;
  if (heating !== undefined && cooling !== undefined && cooling < heating) throw new Error(`energy: ${id} cools below its heating set point`);
  for (const key of ["heating_setpoint", "cooling_setpoint"]) if (row[key] !== undefined && !(row[key] >= -20 && row[key] <= 50)) throw new Error(`energy: ${id} ${key} is out of range`);
  for (const key of ["occupancy_density", "ventilation_rate", "lighting_power_density", "equipment_power_density"]) if (row[key] !== undefined && !(row[key] >= 0)) throw new Error(`energy: ${id} ${key} is negative`);
};

export function energyFor(name: string, model: Json): void {
  for (const [id, row] of Object.entries<Json>(model.window_types)) {
    const [u, g, fraction] = WINDOWS[id] ?? [1.1, 0.5, 0.25];
    Object.assign(row, { u_value: u, g_value: g, frame_fraction: fraction });
  }
  for (const [id, row] of Object.entries<Json>(model.door_types)) row.u_value = DOORS[id] ?? 1.8;
  const rows = name === "house" ? HOUSE : Object.assign({}, ...[0, 1, 2, 3].map(office));
  model.space_conditions = {};
  for (const [id, row] of Object.entries<Json>(rows)) {
    if (model.spaces[id] === undefined) throw new Error(`energy: conditions name the space ${id}, which does not exist`);
    check(id, row);
    model.space_conditions[id] = row;
  }
}
