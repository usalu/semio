/** ➕️ `insert-pier-base` wire twin: the leaf payload `InsertPierBase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998PierBase, parseEn1998PierBase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPierBase {
  mutation: "insertPierBase";
  index?: number | null;
  pierBase: En1998PierBase;
}

export const parseInsertPierBase: NormWireReader<InsertPierBase> = normWireObject<InsertPierBase>({ mutation: normWireRequired(normWireLiteral("insertPierBase")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), pierBase: normWireRequired(parseEn1998PierBase) });
