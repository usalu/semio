/** 🌾 `insert-bulk-solid` wire twin: the leaf payload `InsertBulkSolid`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BulkSolid, parseBulkSolid } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBulkSolid {
  index: number;
  item: BulkSolid;
}

export const parseInsertBulkSolid: NormWireReader<InsertBulkSolid> = normWireObject<InsertBulkSolid>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseBulkSolid) });
