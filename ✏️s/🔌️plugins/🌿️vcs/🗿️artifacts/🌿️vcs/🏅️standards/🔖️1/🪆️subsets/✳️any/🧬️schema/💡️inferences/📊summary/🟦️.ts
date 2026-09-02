/** 📊 `summary` — one named inference: a scalar digest of the tags/notes free-form fields. */

export interface VcsSummary {
  tagCount: number;
  notesWordCount: number;
  hasNotes: boolean;
}
