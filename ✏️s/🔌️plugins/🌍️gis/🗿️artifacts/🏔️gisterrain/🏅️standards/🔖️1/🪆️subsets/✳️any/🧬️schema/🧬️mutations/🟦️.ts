/** 🏔️ GIS terrain direct mutation aggregate. */
import type { ChangeExaggeration } from "./🎚change-exaggeration/🟦️.ts";
import type { ChangeImportedFeatures } from "./📥change-imported-features/🟦️.ts";

export type GisTerrainMutation =
  | { ChangeExaggeration: ChangeExaggeration }
  | { ChangeImportedFeatures: ChangeImportedFeatures };
