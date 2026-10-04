/** 🛢️ `insert-tank` wire twin: the leaf payload `InsertTank`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Tank, parseEn1998Tank } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTank {
  mutation: "insertTank";
  index: number;
  tank: En1998Tank;
}

export const parseInsertTank: NormWireReader<InsertTank> = normWireObject<InsertTank>({ mutation: normWireRequired(normWireLiteral("insertTank")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), tank: normWireRequired(parseEn1998Tank) });
