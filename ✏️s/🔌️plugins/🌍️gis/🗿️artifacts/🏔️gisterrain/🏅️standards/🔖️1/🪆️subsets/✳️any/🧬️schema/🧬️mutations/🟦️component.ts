/** 🏔️ GIS terrain direct mutation aggregate. */
import type { ChangeExaggeration } from "./🎚change-exaggeration/🟦️component.ts";
import type { ChangeImportedFeatures } from "./📥change-imported-features/🟦️component.ts";

export type GisTerrainMutation =
  | { ChangeExaggeration: ChangeExaggeration }
  | { ChangeImportedFeatures: ChangeImportedFeatures };
