/** 🧬 `remove-tendon-point` wire twin: the leaf payload `RemoveTendonPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTendonPoint {
  memberId: string;
  index: number;
}

export const parseRemoveTendonPoint: NormWireReader<RemoveTendonPoint> = normWireObject<RemoveTendonPoint>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
