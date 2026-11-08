/** 📍️ `change-critical-region-section-width-m` wire twin: the leaf payload `ChangeCriticalRegionSectionWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionSectionWidthM {
  mutation: "changeCriticalRegionSectionWidthM";
  id: string;
  newSectionWidthM: number;
}

export const parseChangeCriticalRegionSectionWidthM: NormWireReader<ChangeCriticalRegionSectionWidthM> = normWireObject<ChangeCriticalRegionSectionWidthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionSectionWidthM")), id: normWireRequired(normWireString), newSectionWidthM: normWireRequired(normWireNumber) });
