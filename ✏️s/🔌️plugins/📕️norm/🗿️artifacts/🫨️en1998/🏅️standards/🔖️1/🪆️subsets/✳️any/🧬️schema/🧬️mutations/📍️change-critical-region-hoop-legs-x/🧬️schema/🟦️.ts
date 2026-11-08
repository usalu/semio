/** 📍️ `change-critical-region-hoop-legs-x` wire twin: the leaf payload `ChangeCriticalRegionHoopLegsX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopLegsX {
  mutation: "changeCriticalRegionHoopLegsX";
  id: string;
  newHoopLegsX: number;
}

export const parseChangeCriticalRegionHoopLegsX: NormWireReader<ChangeCriticalRegionHoopLegsX> = normWireObject<ChangeCriticalRegionHoopLegsX>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopLegsX")), id: normWireRequired(normWireString), newHoopLegsX: normWireRequired(normWireNumber) });
