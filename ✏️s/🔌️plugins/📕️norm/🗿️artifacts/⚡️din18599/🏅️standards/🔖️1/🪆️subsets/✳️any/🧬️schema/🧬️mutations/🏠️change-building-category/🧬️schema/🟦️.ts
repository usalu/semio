/** 🏠️ `change-building-category` wire twin: the leaf payload `ChangeBuildingCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599BuildingCategory, parseDin18599BuildingCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBuildingCategory {
  mutation: "changeBuildingCategory";
  newBuildingCategory: Din18599BuildingCategory;
}

export const parseChangeBuildingCategory: NormWireReader<ChangeBuildingCategory> = normWireObject<ChangeBuildingCategory>({ mutation: normWireRequired(normWireLiteral("changeBuildingCategory")), newBuildingCategory: normWireRequired(parseDin18599BuildingCategory) });
