/** 📥 `insert-pile` wire twin: the leaf payload `InsertPile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997Pile, parseEn1997Pile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPile {
  mutation: "insertPile";
  index: number;
  pile: En1997Pile;
}

export const parseInsertPile: NormWireReader<InsertPile> = normWireObject<InsertPile>({ mutation: normWireRequired(normWireLiteral("insertPile")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pile: normWireRequired(parseEn1997Pile) });
