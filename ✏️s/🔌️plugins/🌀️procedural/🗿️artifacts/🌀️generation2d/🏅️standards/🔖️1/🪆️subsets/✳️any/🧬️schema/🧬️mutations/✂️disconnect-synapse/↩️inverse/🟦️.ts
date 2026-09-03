/** ↩️ generation2d disconnect-synapse inverse — mirrors `inverse()` (…/✂️disconnect-synapse/↩️inverse/🦀️.rs): reconnects the removed edge at its captured BASE index, or a no-op when the id was already absent. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️.ts";
import type { ConnectSynapse, SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️.ts";

export function inverse(_payload: DisconnectSynapse, baseSynapse: { index: number; synapse: SynapseSpec } | undefined): ConnectSynapse[] {
  return baseSynapse ? [{ index: baseSynapse.index, synapse: baseSynapse.synapse }] : [];
}
