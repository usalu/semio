/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { core } from "@semio-tech/cad-js";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-module-aec-building-structure/🟦️.ts`. */
export type AecBuildingStructureTestDependencies = {
  readonly AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID: string;
  readonly STRUCTURE_FROM_BUILDING_TRANSFORMATION_ID: string;
  readonly STRUCTURE_STABILITY_STAT_ID: string;
  readonly core: typeof core;
  readonly qualifiedTransformationId: typeof core.qualifiedTransformationId;
  readonly solidRef: typeof core.solidRef;
};
