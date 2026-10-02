/** 🏞️ `change-terrain-category` wire twin: the leaf payload `ChangeTerrainCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTerrainCategory {
  newTerrainCategory: number;
}

export const parseChangeTerrainCategory: NormWireReader<ChangeTerrainCategory> = normWireObject<ChangeTerrainCategory>({ newTerrainCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4})) });
