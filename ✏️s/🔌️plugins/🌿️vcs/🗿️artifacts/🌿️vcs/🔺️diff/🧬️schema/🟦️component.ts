/** 🧬️ VCS diff schema — sparse field delta over the artifact. */

export interface VcsDiff {
  /** @state persistent */
  artifact?: VcsArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  counter?: number;
  /** @state persistent */
  notes?: string;
  /** @state persistent */
  status?: string;
  /** @state persistent */
  tags?: VcsTagsDelta;
  /** @state shared-ui */
  selectedCheckpointIds?: VcsStringList;
  /** @state local-ui */
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
