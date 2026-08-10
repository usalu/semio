/** 🧬️ VCS artifact schema — every field with its state class. */

export interface VcsArtifact {
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
  /** @state shared-ui */
  selectedCheckpointIds: string[];
  /** @state local-ui */
  locale: string;
}
