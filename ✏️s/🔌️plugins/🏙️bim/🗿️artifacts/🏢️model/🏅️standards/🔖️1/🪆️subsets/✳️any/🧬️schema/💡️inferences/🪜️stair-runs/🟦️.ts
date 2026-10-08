/** 🪜️ `stair-runs`: the resolved run of a stair; the foot of riser `k` of a flight stands at `start + k * tread` along `direction` and rises `riser_height` per riser. */

export interface Point2 {
  x: number;
  y: number;
}

export interface StairWinder {
  centre: Point2;
  inner_radius: number;
  outer_radius: number;
  start_angle: number;
  sweep: number;
}

export interface StairFlightRun {
  first_riser: number;
  risers: number;
  treads: number;
  start: Point2;
  direction: number;
  tread: number;
  base_z: number;
  length: number;
  winder?: StairWinder;
}

export interface StairLanding {
  after_flight: number;
  z: number;
  centre: Point2;
  direction: number;
  width: number;
  depth: number;
}

export interface StairCompliance {
  rise_positive: boolean;
  riser_ok: boolean;
  tread_ok: boolean;
  blondel_ok: boolean;
  compliant: boolean;
}

export interface StairRun {
  base_z: number;
  top_z: number;
  rise: number;
  riser_count: number;
  riser_height: number;
  tread_count: number;
  tread: number;
  stride: number;
  width: number;
  run_length: number;
  flights: StairFlightRun[];
  landings: StairLanding[];
  compliance: StairCompliance;
}
