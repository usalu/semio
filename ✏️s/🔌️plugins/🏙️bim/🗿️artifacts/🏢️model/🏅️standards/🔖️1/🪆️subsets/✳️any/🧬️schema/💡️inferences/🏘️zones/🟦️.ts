/** 🏘️ `zones`: what the zones and the area schemes add up over the rooms of their spaces. */

export interface ZoneTotals {
  spaces: number;
  resolved: number;
  area: number;
  net_area: number;
  volume: number;
  occupancy: number;
  floor_finish_area: number;
  wall_finish_area: number;
  ceiling_finish_area: number;
}

export interface SchemeTotals {
  spaces: number;
  resolved: number;
  area: number;
  volume: number;
  occupancy: number;
}
