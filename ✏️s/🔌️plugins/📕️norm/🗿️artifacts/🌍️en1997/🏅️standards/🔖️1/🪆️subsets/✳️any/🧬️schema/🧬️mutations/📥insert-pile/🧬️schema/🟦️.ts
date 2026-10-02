/** 📥 `insert-pile` wire twin: the leaf payload `InsertPile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parsePile, type Pile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPile {
  mutation: "insertPile";
  index: number;
  pile: Pile;
}

export const parseInsertPile: NormWireReader<InsertPile> = normWireObject<InsertPile>({ mutation: normWireRequired(normWireLiteral("insertPile")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pile: normWireRequired(parsePile) });
