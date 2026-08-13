/** 🧬️ VCS snapshot schema — artifact-lane fields only. */

export interface VcsSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  title: string;
  /** @state artifact */
  counter: number;
  /** @state artifact */
  notes: string;
  /** @state artifact */
  status: string;
  /** @state artifact */
  tags: string[];
}
