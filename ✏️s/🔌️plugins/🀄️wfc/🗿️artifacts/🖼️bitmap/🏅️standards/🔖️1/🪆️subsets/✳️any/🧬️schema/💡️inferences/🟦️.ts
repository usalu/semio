/** 💡️ Bitmap solve inference — derived from the persisted problem, never stored on it. */
export interface BitmapInferenceCommit {
  /** Base64 of one palette index per output pixel, row-major. Empty on a contradiction. */
  pixels: string;
  contradiction: boolean;
  /** Per-output-cell prior Shannon entropy, row-major. */
  entropy: number[];
}
