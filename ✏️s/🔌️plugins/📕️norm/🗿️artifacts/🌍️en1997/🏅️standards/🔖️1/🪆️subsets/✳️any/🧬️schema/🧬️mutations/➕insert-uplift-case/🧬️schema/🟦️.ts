/** ➕ `insert-uplift-case` wire twin: the leaf payload `InsertUpliftCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997UpliftCase, parseEn1997UpliftCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertUpliftCase {
  mutation: "insertUpliftCase";
  index: number;
  upliftCase: En1997UpliftCase;
}

export const parseInsertUpliftCase: NormWireReader<InsertUpliftCase> = normWireObject<InsertUpliftCase>({ mutation: normWireRequired(normWireLiteral("insertUpliftCase")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), upliftCase: normWireRequired(parseEn1997UpliftCase) });
