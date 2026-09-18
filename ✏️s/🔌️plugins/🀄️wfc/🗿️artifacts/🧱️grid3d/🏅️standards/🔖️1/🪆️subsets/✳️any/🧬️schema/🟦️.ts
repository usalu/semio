/** 🧬 s.wfc.grid3d artifact facet — the persisted problem spec IS the artifact. */

import type { Grid3dSnapshot } from "./📸️snapshot/🟦️.ts";

export interface Grid3dArtifact {
  snapshot: Grid3dSnapshot;
}
