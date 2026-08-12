/** 💡️ VCS inference schema — a scalar summary digest of the tags/notes free-form fields. */

export interface VcsSummary {
  tagCount: number;
  notesWordCount: number;
  hasNotes: boolean;
}

export interface VcsInference {
  /** @state inferred */
  summary: VcsSummary;
}
