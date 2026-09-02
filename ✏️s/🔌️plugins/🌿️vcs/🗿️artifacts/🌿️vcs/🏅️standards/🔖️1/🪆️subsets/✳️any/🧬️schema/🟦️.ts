/** 🧬️ VCS artifact schema — every field with its state class. */

export interface VcsArtifact {
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
  /** @state presence */
  selectedCheckpointIds: string[];
  /** @state config */
  locale: string;
}
