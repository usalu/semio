/** 🧬️ VCS diff schema — sparse field delta over the artifact. */

export interface VcsDiff {
  /** @state artifact */
  artifact?: VcsArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  counter?: number;
  /** @state artifact */
  notes?: string;
  /** @state artifact */
  status?: string;
  /** @state artifact */
  tags?: VcsTagsDelta;
  /** @state presence */
  selectedCheckpointIds?: VcsStringList;
  /** @state config */
  locale?: string;
}

export interface VcsStringList {
  values: string[];
}

export interface VcsTagsDelta {
  added: string[];
  removed: string[];
}

export interface VcsArtifact {
  schema: string;
  title: string;
  counter: number;
  notes: string;
  status: string;
  tags: string[];
  selectedCheckpointIds: string[];
  locale: string;
}
