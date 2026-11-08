/** 📍️ `change-critical-region-core-depth-m` wire twin: the leaf payload `ChangeCriticalRegionCoreDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionCoreDepthM {
  mutation: "changeCriticalRegionCoreDepthM";
  id: string;
  newCoreDepthM: number;
}

export const parseChangeCriticalRegionCoreDepthM: NormWireReader<ChangeCriticalRegionCoreDepthM> = normWireObject<ChangeCriticalRegionCoreDepthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionCoreDepthM")), id: normWireRequired(normWireString), newCoreDepthM: normWireRequired(normWireNumber) });
