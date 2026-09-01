/** 🔗 procedural3d direct `connect-synapse` payload mirror of `ConnectSynapse`. */
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
