/** 📍️ `change-critical-region-clear-length-m` wire twin: the leaf payload `ChangeCriticalRegionClearLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionClearLengthM {
  mutation: "changeCriticalRegionClearLengthM";
  id: string;
  newClearLengthM: number;
}

export const parseChangeCriticalRegionClearLengthM: NormWireReader<ChangeCriticalRegionClearLengthM> = normWireObject<ChangeCriticalRegionClearLengthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionClearLengthM")), id: normWireRequired(normWireString), newClearLengthM: normWireRequired(normWireNumber) });
