/** 📍️ `change-critical-region-hoop-legs-y` wire twin: the leaf payload `ChangeCriticalRegionHoopLegsY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopLegsY {
  mutation: "changeCriticalRegionHoopLegsY";
  id: string;
  newHoopLegsY: number;
}

export const parseChangeCriticalRegionHoopLegsY: NormWireReader<ChangeCriticalRegionHoopLegsY> = normWireObject<ChangeCriticalRegionHoopLegsY>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopLegsY")), id: normWireRequired(normWireString), newHoopLegsY: normWireRequired(normWireNumber) });
