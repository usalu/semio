/** 📍️ `change-critical-region-section-depth-m` wire twin: the leaf payload `ChangeCriticalRegionSectionDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionSectionDepthM {
  mutation: "changeCriticalRegionSectionDepthM";
  id: string;
  newSectionDepthM: number;
}

export const parseChangeCriticalRegionSectionDepthM: NormWireReader<ChangeCriticalRegionSectionDepthM> = normWireObject<ChangeCriticalRegionSectionDepthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionSectionDepthM")), id: normWireRequired(normWireString), newSectionDepthM: normWireRequired(normWireNumber) });
