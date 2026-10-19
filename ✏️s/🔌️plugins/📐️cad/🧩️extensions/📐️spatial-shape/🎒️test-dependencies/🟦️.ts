/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { core } from "@semio-tech/cad-js";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts`. */
export type SpatialShapeTestDependencies = {
  readonly SPATIAL_SHAPE_GEOMETRY_STAT_ID: string;
  readonly SPATIAL_SHAPE_VOLUME_PROPERTY_ID: string;
  readonly core: typeof core;
  readonly solidRef: typeof core.solidRef;
};
