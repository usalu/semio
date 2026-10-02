/** 💣 `insert-accidental` wire twin: the leaf payload `InsertAccidental`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990AccidentalAction, parseEn1990AccidentalAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAccidental {
  mutation: "insertAccidental";
  index: number;
  item: En1990AccidentalAction;
}

export const parseInsertAccidental: NormWireReader<InsertAccidental> = normWireObject<InsertAccidental>({ mutation: normWireRequired(normWireLiteral("insertAccidental")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990AccidentalAction) });
