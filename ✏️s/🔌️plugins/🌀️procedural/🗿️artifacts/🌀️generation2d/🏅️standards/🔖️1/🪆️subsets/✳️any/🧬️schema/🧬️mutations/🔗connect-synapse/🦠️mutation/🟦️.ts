/** 🔗 generation2d connect-synapse payload — mirrors `ConnectSynapse` (…/🔗connect-synapse/🦠️mutation/🦀️.rs:16-19). */
export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

export interface ConnectSynapse {
  index: number;
  synapse: SynapseSpec;
}
