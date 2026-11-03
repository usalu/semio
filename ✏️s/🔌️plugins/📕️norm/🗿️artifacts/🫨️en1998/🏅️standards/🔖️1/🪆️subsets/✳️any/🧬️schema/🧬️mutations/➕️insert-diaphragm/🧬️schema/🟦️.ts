/** ➕️ `insert-diaphragm` wire twin: the leaf payload `InsertDiaphragm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Diaphragm, parseEn1998Diaphragm } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDiaphragm {
  mutation: "insertDiaphragm";
  index?: number | null;
  diaphragm: En1998Diaphragm;
}

export const parseInsertDiaphragm: NormWireReader<InsertDiaphragm> = normWireObject<InsertDiaphragm>({ mutation: normWireRequired(normWireLiteral("insertDiaphragm")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), diaphragm: normWireRequired(parseEn1998Diaphragm) });
