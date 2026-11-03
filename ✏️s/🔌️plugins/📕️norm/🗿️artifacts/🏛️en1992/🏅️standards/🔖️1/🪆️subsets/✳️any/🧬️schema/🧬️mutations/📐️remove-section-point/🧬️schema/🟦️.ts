/** 📐️ `remove-section-point` wire twin: the leaf payload `RemoveSectionPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSectionPoint {
  memberId: string;
  index: number;
}

export const parseRemoveSectionPoint: NormWireReader<RemoveSectionPoint> = normWireObject<RemoveSectionPoint>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
