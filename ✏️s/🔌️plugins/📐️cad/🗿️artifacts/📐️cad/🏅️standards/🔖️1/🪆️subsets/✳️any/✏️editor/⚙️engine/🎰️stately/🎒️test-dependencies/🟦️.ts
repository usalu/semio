/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { init, macrostep, NullInspector, Machine } from "@semio-tech/framework/machine";
import type { emptyMeshTransfer, solidRef } from "@semio-tech/framework/3d";
import type { InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import type { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { listSpatialInteractionsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import type { isEmptyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { pureTsStateEngineProvider, StateEngineProvider } from "../../🎬️actions/🎰️interaction-state/🟦️.ts";
import type { loadSpatialInteraction } from "../../🗿️artifact/📚️interaction-catalog/🟦️.ts";
import type { createInteractionRuntime } from "../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import type { StatelyAdvanceEvent, StatelyMachineSpec, SpatialStatelyMachineCatalogView } from "../🎛️machine-views/🟦️.ts";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts`. */
export type StatelyTestDependencies = {
  readonly Model: typeof Model;
  readonly NullInspector: typeof NullInspector;
  readonly __spatialStatelyTestKernel: typeof import("@semio-tech/cad-js/testing/spatial-kernel/brepjs") | null;
  readonly __spatialStatelyTestRuntime: typeof import("../../🏃️runtime/🟦️.ts") | null;
  readonly buildSpatialStatelyMachineCatalogView: (opts: { readonly modelDefinitionId: string; readonly interactionIds?: readonly string[]; readonly generatedAt?: string }) => SpatialStatelyMachineCatalogView;
  readonly buildStatelyMachine: (spec: InteractionSpec, initial: string) => Machine<StatelyMachineSpec>;
  readonly createInteractionRuntime: typeof createInteractionRuntime;
  readonly defaultModelDefinitionId: typeof defaultModelDefinitionId;
  readonly emptyMeshTransfer: typeof emptyMeshTransfer;
  readonly init: typeof init;
  readonly isEmptyModelDiff: typeof isEmptyModelDiff;
  readonly listSpatialInteractionsForModelDefinition: typeof listSpatialInteractionsForModelDefinition;
  readonly loadSpatialInteraction: typeof loadSpatialInteraction;
  readonly macrostep: typeof macrostep;
  readonly makeAdvanceEvent: (interactionKind: string, branch: number) => StatelyAdvanceEvent;
  readonly pureTsStateEngineProvider: typeof pureTsStateEngineProvider;
  readonly solidRef: typeof solidRef;
  readonly statelyStateEngineProvider: StateEngineProvider;
};
