/** 🧬️ wfc3d artifact schema — the persisted WFC problem spec is the artifact. */

export { type Wfc3dSnapshot, type Slot3d, type SlotEdge, type Tile, type TileMedia3d, type GraphRule, type Color, WFC3D_DOCUMENT_SCHEMA } from "./📸️snapshot/🟦️";

export interface Wfc3dArtifact {
  /** @state artifact */
  snapshot: import("./📸️snapshot/🟦️").Wfc3dSnapshot;
}
