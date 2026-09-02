/** 🔺️ procedural3d update-synapse/🔺️diff — mirror of the whole-body synapse patch delta builder. */
import type { UpdateSynapse } from "../🦠️mutation/🟦️.ts";
import type { SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️.ts";

export function diff(payload: UpdateSynapse): { synapses: { removed: string[]; set: Array<[number, SynapseSpec]> } } {
  return { synapses: { removed: [], set: [[0, payload.synapse]] } };
}
