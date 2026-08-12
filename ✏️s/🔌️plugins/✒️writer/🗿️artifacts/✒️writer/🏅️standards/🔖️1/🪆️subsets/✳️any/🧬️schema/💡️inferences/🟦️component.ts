/** 💡️ Writer inference schema — document outline (markdown headings + word/line counts). */

export interface WriterOutline {
  sectionOutline: string[];
  wordCount: number;
  lineCount: number;
}

export interface WriterInference {
  /** @state inferred */
  outline: WriterOutline;
}
