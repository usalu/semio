/** 🎛️ The stately machine spec and the transition, state, machine and catalog views the stately engine publishes. */

import type { MachineSpec, StatechartEvent } from "@semio-tech/framework/machine";

/** 🎭️ The one wire-level event kind every compiled `StatelyMachineSpec` reacts to; rows for
 * different `spec.machine` `(state, event)` pairs are disambiguated purely by guard, not by event
 * identity — same encoding the former XState chart used (`__advance` + `interactionKind` + `branch`). */
export interface StatelyAdvanceEvent extends StatechartEvent {
  readonly type: "__advance";
  readonly interactionKind: string;
  readonly branch: number;
}

/** 🎭️ Associated types bound to `@semio-tech/machine`'s generic kernel for the stately adapter. */
export interface StatelyMachineSpec extends MachineSpec {
  Context: undefined;
  Event: StatelyAdvanceEvent;
  Input: undefined;
  Output: never;
  Effect: never;
}

/** 📊️ One transition row for `🔣️machine.json` / Mermaid (matches `__advance` branch order). */
export interface SpatialStatelyMachineTransitionView {
  readonly from: string;
  readonly to: string;
  readonly on: string;
  readonly branch: number;
  readonly guard: string | null;
  readonly transient: boolean;
  readonly key?: string;
  readonly label?: string;
}

/** 📊️ Serializable state node summary. */
export interface SpatialStatelyMachineStateView {
  readonly id: string;
  readonly final: boolean;
  readonly selectionAccept?: readonly string[];
}

/** 📊️ Single spatial interaction as a viewable state machine (edges + Mermaid). */
export interface SpatialStatelyMachineView {
  readonly interactionId: string;
  readonly interactionVersion: string;
  readonly label: string;
  readonly hostKey: string;
  readonly initial: string;
  readonly states: readonly SpatialStatelyMachineStateView[];
  readonly edges: readonly SpatialStatelyMachineTransitionView[];
  readonly mermaid: string;
  readonly statelyRoutingNote: string;
}

/** 📊️ Catalog of model-definition interactions for Stately/Mermaid viewers (`🔣️machine.json`). */
export interface SpatialStatelyMachineCatalogView {
  readonly kind: "spatial.stately-machine-view/v1";
  readonly schemaVersion: "1.0";
  readonly generatedAt: string;
  readonly machines: readonly SpatialStatelyMachineView[];
  readonly mermaidCombined: string;
}
