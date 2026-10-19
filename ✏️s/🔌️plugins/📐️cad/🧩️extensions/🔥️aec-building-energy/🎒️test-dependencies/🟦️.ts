/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { core } from "@semio-tech/cad-js";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-module-aec-building-energy/🟦️.ts`. */
export type AecBuildingEnergyTestDependencies = {
  readonly AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID: string;
  readonly ENERGY_DEMAND_STAT_ID: string;
  readonly core: typeof core;
  readonly solidRef: typeof core.solidRef;
};
