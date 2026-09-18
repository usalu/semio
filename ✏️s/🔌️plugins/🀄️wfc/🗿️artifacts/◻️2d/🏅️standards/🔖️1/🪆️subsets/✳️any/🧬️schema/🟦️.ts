// 🧬️ WFC 2D artifact facet — the TypeScript twin of `🦀️.rs`, ported field by field.

import type { Wfc2dSnapshot } from "./📸️snapshot/🟦️.ts";
import { defaultWfc2dSnapshot } from "./📸️snapshot/🟦️.ts";

/** 🧬️ The artifact facet — the persisted problem spec IS the artifact. */
export type Wfc2dArtifact = { readonly snapshot: Wfc2dSnapshot };

export const WFC_2D_ARTIFACT_SCHEMA_ID = "s.wfc.wfc2d";

/** 🌱️ The empty artifact this subset boots with. */
export function defaultWfc2dArtifact(): Wfc2dArtifact {
  return { snapshot: defaultWfc2dSnapshot() };
}
