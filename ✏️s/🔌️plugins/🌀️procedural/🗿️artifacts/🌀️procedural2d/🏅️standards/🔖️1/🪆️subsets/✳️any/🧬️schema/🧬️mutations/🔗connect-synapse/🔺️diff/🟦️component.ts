/** 🔺️ procedural2d connect-synapse diff — mirrors `diff()` (…/🔗connect-synapse/🔺️diff/🦀️component.rs), a sparse insert into the fixture's synapse collection. */
import type { ConnectSynapse, SynapseSpec } from "../🦠️mutation/🟦️component.ts";

export interface ConnectSynapseDiff {
  synapses: { removed: string[]; set: Array<[number, SynapseSpec]> };
}

export function diff(payload: ConnectSynapse): ConnectSynapseDiff {
  return { synapses: { removed: [], set: [[payload.index, payload.synapse]] } };
}
