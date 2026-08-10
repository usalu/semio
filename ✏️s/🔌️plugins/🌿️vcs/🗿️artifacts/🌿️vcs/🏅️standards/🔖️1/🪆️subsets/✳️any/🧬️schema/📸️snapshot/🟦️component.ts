/** 🧬️ VCS snapshot schema — persistent fields only. */

export interface VcsSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  title: string;
  /** @state persistent */
  counter: number;
  /** @state persistent */
  notes: string;
  /** @state persistent */
  status: string;
  /** @state persistent */
  tags: string[];
}
