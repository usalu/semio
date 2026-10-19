/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { DisplayModel } from "../🖼️display-model/🟦️.ts";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts`. */
export type ActionsTestDependencies = {
  readonly __actionsTestRuntime: typeof import("../../🏃️runtime/🟦️.ts") | null;
  readonly resolveDisplay: (spec: InteractionSpec, state: string, context: Record<string, unknown>, preview: SpatialPreviewKernel, model?: Model) => DisplayModel;
};
