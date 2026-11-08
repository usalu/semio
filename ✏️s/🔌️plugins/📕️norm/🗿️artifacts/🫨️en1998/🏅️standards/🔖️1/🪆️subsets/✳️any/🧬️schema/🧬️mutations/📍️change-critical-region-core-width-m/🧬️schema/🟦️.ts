/** 📍️ `change-critical-region-core-width-m` wire twin: the leaf payload `ChangeCriticalRegionCoreWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionCoreWidthM {
  mutation: "changeCriticalRegionCoreWidthM";
  id: string;
  newCoreWidthM: number;
}

export const parseChangeCriticalRegionCoreWidthM: NormWireReader<ChangeCriticalRegionCoreWidthM> = normWireObject<ChangeCriticalRegionCoreWidthM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionCoreWidthM")), id: normWireRequired(normWireString), newCoreWidthM: normWireRequired(normWireNumber) });
