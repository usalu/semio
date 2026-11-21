/** 🏔️ GIS terrain direct mutation aggregate. */
import type { ChangeExaggeration } from "./🎚️change-exaggeration/🟦️.ts";
import type { ChangeImportedFeatures } from "./📥change-imported-features/🟦️.ts";
import type { ReplaceElevationSource } from "./🏔️replace-elevation-source/🟦️.ts";

export type GisTerrainMutation =
  | { ChangeExaggeration: ChangeExaggeration }
  | { ChangeImportedFeatures: ChangeImportedFeatures }
  | { ReplaceElevationSource: ReplaceElevationSource };
