/** ➕️ `insert-fastener-position` wire twin: the leaf payload `InsertFastenerPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FastenerPosition, parseFastenerPosition } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFastenerPosition {
  fastenerId: string;
  index: number;
  position: FastenerPosition;
}

export const parseInsertFastenerPosition: NormWireReader<InsertFastenerPosition> = normWireObject<InsertFastenerPosition>({ fastenerId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), position: normWireRequired(parseFastenerPosition) });
