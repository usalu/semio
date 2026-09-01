/** ↩️ procedural3d disconnect-synapse/↩️inverse — mirror of the BASE-lookup recreate-synapse inverse. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️component.ts";
import type { ConnectSynapse, SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️component.ts";

export function inverse(_payload: DisconnectSynapse, baseSynapse: { index: number; synapse: SynapseSpec } | undefined): ConnectSynapse[] {
  return baseSynapse === undefined ? [] : [{ index: baseSynapse.index, synapse: baseSynapse.synapse }];
}
