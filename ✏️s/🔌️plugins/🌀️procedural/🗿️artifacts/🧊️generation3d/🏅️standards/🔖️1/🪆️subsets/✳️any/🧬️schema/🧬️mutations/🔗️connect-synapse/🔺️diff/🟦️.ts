/** 🔺️ generation3d connect-synapse/🔺️diff — mirror of the append-only synapses-set delta builder. */
import type { ConnectSynapse, SynapseSpec } from "../🦠️mutation/🟦️.ts";

export function diff(payload: ConnectSynapse): { synapses: { removed: string[]; set: Array<[number, SynapseSpec]> } } {
  return { synapses: { removed: [], set: [[payload.index, payload.synapse]] } };
}
