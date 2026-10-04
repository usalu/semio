/** 📥️ `change-imported-features` mutation payload — sets the terrain's last-imported `2d.map` descriptor JSON (the `map:in` insertion point). */
import type {ImportedMap} from "../../🗺️imported-map/🟦️.ts";
export interface ChangeImportedFeatures {
  newImportedMap?: ImportedMap;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`imported-features` kind=`change-imported-features` record=`ChangedImportedFeatures`. */
export const ChangeImportedFeaturesKind = "change-imported-features" as const;
