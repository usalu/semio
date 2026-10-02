/** ➕️ `insert-material` wire twin: the leaf payload `InsertMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelMaterial, type SteelMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMaterial {
  index: number;
  material: SteelMaterial;
}

export const parseInsertMaterial: NormWireReader<InsertMaterial> = normWireObject<InsertMaterial>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), material: normWireRequired(parseSteelMaterial) });
