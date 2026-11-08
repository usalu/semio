/** 📍️ `change-critical-region-restr-bar-count` wire twin: the leaf payload `ChangeCriticalRegionRestrainedBarCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionRestrainedBarCount {
  mutation: "changeCriticalRegionRestrainedBarCount";
  id: string;
  newRestrainedBarCount: number;
}

export const parseChangeCriticalRegionRestrainedBarCount: NormWireReader<ChangeCriticalRegionRestrainedBarCount> = normWireObject<ChangeCriticalRegionRestrainedBarCount>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionRestrainedBarCount")), id: normWireRequired(normWireString), newRestrainedBarCount: normWireRequired(normWireNumber) });
