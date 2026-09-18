/** 🧬️ Bitmap artifact schema — the persisted overlapping-model problem. */

export { type BitmapColor, type BitmapInput, type BitmapOutputSpec, type BitmapOverlappingModel, type BitmapPinnedPixel, type BitmapSnapshot } from "./📸️snapshot/🟦️";

export interface BitmapArtifact {
  /** @state artifact */
  snapshot: import("./📸️snapshot/🟦️").BitmapSnapshot;
}
