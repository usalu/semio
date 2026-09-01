/** ↩️ procedural2d disconnect-synapse inverse — mirrors `inverse()` (…/✂️disconnect-synapse/↩️inverse/🦀️component.rs): reconnects the removed edge at its captured BASE index, or a no-op when the id was already absent. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️component.ts";
import type { ConnectSynapse, SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️component.ts";

export function inverse(_payload: DisconnectSynapse, baseSynapse: { index: number; synapse: SynapseSpec } | undefined): ConnectSynapse[] {
  return baseSynapse ? [{ index: baseSynapse.index, synapse: baseSynapse.synapse }] : [];
}
