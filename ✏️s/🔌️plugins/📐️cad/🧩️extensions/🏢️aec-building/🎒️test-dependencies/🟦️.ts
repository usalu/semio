/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { core } from "@semio-tech/cad-js";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-module-aec-building/🟦️.ts`. */
export type AecBuildingTestDependencies = {
  readonly AEC_BUILDING_MODEL_DEFINITION_ID: string;
  readonly typologyFromStepLayer: typeof core.typologyFromStepLayer;
};
