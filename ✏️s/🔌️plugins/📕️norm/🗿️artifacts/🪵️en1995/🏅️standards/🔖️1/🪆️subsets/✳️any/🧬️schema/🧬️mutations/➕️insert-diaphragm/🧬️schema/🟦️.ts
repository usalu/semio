/** ➕️ `insert-diaphragm` wire twin: the leaf payload `InsertDiaphragm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Diaphragm, parseDiaphragm } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDiaphragm {
  index: number;
  diaphragm: Diaphragm;
}

export const parseInsertDiaphragm: NormWireReader<InsertDiaphragm> = normWireObject<InsertDiaphragm>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), diaphragm: normWireRequired(parseDiaphragm) });
