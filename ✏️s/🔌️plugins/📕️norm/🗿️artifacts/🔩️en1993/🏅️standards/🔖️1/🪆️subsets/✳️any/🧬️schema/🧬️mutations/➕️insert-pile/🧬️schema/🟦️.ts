/** ➕️ `insert-pile` wire twin: the leaf payload `InsertPile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelPile, type SteelPile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPile {
  index: number;
  pile: SteelPile;
}

export const parseInsertPile: NormWireReader<InsertPile> = normWireObject<InsertPile>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pile: normWireRequired(parseSteelPile) });
