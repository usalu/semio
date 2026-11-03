/** 🧬 `insert-tendon-point` wire twin: the leaf payload `InsertTendonPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TendonPoint, parseTendonPoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTendonPoint {
  memberId: string;
  index: number;
  tendonPoint: TendonPoint;
}

export const parseInsertTendonPoint: NormWireReader<InsertTendonPoint> = normWireObject<InsertTendonPoint>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), tendonPoint: normWireRequired(parseTendonPoint) });
