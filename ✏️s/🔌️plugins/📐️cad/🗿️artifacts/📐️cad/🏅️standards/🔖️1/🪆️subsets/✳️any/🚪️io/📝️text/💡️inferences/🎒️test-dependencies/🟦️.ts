/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { emptyMeshTransfer, solidRef } from "@semio-tech/framework/3d";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { modelDefinitionActionRegistry, ConstructQueryContext, ConstructQueryResult } from "../../../../✏️editor/⚙️engine/🎬️actions/🏃️action-execution/🟦️.ts";
import type { runConstructAst, ConstructAst } from "../../../../🧬️schema/💡️inferences/🟦️.ts";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts`. */
export type InferencesTestDependencies = {
  readonly Model: typeof Model;
  readonly __spatialQueryTestKernel: typeof import("@semio-tech/cad-js/testing/spatial-kernel/brepjs") | null;
  readonly __spatialQueryTestRuntime: typeof import("../../../../✏️editor/⚙️engine/🏃️runtime/🟦️.ts") | null;
  readonly applyModelDiff: typeof applyModelDiff;
  readonly emptyMeshTransfer: typeof emptyMeshTransfer;
  readonly modelDefinitionActionRegistry: typeof modelDefinitionActionRegistry;
  readonly parseConstruct: (text: string, activeModelDefinitionId?: string | null) => ConstructAst;
  readonly runConstruct: (text: string, ctx: ConstructQueryContext) => Promise<ConstructQueryResult>;
  readonly runConstructAst: typeof runConstructAst;
  readonly solidRef: typeof solidRef;
};
