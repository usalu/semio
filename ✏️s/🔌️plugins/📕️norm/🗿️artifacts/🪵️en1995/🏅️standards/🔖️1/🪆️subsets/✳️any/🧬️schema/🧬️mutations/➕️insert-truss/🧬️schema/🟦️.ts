/** ➕️ `insert-truss` wire twin: the leaf payload `InsertTruss`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Truss, parseTruss } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTruss {
  index: number;
  truss: Truss;
}

export const parseInsertTruss: NormWireReader<InsertTruss> = normWireObject<InsertTruss>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), truss: normWireRequired(parseTruss) });
