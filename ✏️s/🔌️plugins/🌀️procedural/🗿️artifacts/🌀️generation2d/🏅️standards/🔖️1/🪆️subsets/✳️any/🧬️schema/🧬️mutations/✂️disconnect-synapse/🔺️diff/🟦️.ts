/** 🔺️ generation2d disconnect-synapse diff — mirrors `diff()` (…/✂️disconnect-synapse/🔺️diff/🦀️.rs), a sparse id-keyed removal from the fixture's synapse collection. */
import type { DisconnectSynapse } from "../🦠️mutation/🟦️.ts";

export interface DisconnectSynapseDiff {
  synapses: { removed: string[]; set: never[] };
}

export function diff(payload: DisconnectSynapse): DisconnectSynapseDiff {
  return { synapses: { removed: [payload.id], set: [] } };
}
