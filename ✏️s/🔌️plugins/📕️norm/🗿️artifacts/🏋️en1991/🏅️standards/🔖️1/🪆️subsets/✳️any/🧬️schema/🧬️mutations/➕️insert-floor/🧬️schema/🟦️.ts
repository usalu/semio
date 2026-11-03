/** ➕️ `insert-floor` wire twin: the leaf payload `InsertFloor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FloorArea, parseFloorArea } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFloor {
  index: number;
  item: FloorArea;
}

export const parseInsertFloor: NormWireReader<InsertFloor> = normWireObject<InsertFloor>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseFloorArea) });
