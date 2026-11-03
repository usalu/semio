/** 💣 `insert-accidental` wire twin: the leaf payload `InsertAccidental`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990AccidentalAction, parseEn1990AccidentalAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAccidental {
  mutation: "insertAccidental";
  index?: number | null;
  item: En1990AccidentalAction;
}

export const parseInsertAccidental: NormWireReader<InsertAccidental> = normWireObject<InsertAccidental>({ mutation: normWireRequired(normWireLiteral("insertAccidental")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), item: normWireRequired(parseEn1990AccidentalAction) });
