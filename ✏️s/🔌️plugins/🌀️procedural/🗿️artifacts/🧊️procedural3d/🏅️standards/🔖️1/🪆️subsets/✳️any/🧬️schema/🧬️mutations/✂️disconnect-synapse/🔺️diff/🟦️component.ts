/** 🔺️ procedural3d disconnect-synapse/🔺️diff — mirror of the id-only synapses-removed delta builder. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️component.ts";
import type { SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️component.ts";

export function diff(payload: DisconnectSynapse): { synapses: { removed: string[]; set: Array<[number, SynapseSpec]> } } {
  return { synapses: { removed: [payload.id], set: [] } };
}
