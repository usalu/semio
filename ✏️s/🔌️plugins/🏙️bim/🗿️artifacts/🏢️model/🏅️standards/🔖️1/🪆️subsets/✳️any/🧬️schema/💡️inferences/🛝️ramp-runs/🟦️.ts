/** 🛝️ ramp-runs: the resolved run of every ramp: rise, length, sloped run, slope, flights, landings and the code flags. */

export interface RampRun {
  base_z: number;
  top_z: number;
  rise: number;
  length: number;
  run_length: number;
  slope: number;
  angle: number;
  width: number;
  flights: RampFlight[];
  landings: RampLanding[];
  compliance: RampCompliance;
}

export interface RampFlight {
  from: number;
  to: number;
  length: number;
  z_from: number;
  z_to: number;
}

export interface RampLanding {
  from: number;
  to: number;
  length: number;
  z: number;
}

export interface RampCompliance {
  run_ok: boolean;
  slope_ok: boolean;
  compliant: boolean;
}
