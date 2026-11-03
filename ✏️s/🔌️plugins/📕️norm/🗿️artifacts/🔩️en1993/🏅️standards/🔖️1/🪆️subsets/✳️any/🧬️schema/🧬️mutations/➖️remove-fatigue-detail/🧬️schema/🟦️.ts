/** ➖️ `remove-fatigue-detail` wire twin: the leaf payload `RemoveFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueDetail {
  index: number;
}

export const parseRemoveFatigueDetail: NormWireReader<RemoveFatigueDetail> = normWireObject<RemoveFatigueDetail>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
