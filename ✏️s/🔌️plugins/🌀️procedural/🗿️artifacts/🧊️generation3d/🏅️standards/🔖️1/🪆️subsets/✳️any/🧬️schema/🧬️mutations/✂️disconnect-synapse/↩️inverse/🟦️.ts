/** ↩️ generation3d disconnect-synapse/↩️inverse — mirror of the BASE-lookup recreate-synapse inverse. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️.ts";
import type { ConnectSynapse, SynapseSpec } from "../../🔗️connect-synapse/🦠️mutation/🟦️.ts";

export function inverse(_payload: DisconnectSynapse, baseSynapse: { index: bigint; synapse: SynapseSpec } | undefined): ConnectSynapse[] {
  return baseSynapse === undefined ? [] : [{ index: baseSynapse.index, synapse: baseSynapse.synapse }];
}
