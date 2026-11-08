/** 📍️ `change-critical-region-hooped-length-m` wire twin: the leaf payload `ChangeCriticalRegionHoopedLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopedLengthM {
  mutation: "changeCriticalRegionHoopedLengthM";
  id: string;
  newHoopedLengthM: number;
}

export const parseChangeCriticalRegionHoopedLengthM: NormWireReader<ChangeCriticalRegionHoopedLengthM> = normWireObject<ChangeCriticalRegionHoopedLengthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopedLengthM")), id: normWireRequired(normWireString), newHoopedLengthM: normWireRequired(normWireNumber) });
