/** 📍 `insert-bar-point` wire twin: the leaf payload `InsertBarPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BarPoint, parseBarPoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBarPoint {
  memberId: string;
  index: number;
  barPoint: BarPoint;
}

export const parseInsertBarPoint: NormWireReader<InsertBarPoint> = normWireObject<InsertBarPoint>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), barPoint: normWireRequired(parseBarPoint) });
