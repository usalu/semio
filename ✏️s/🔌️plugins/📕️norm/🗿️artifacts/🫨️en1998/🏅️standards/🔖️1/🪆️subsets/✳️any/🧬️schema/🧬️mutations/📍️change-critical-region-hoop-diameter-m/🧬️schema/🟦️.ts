/** 📍️ `change-critical-region-hoop-diameter-m` wire twin: the leaf payload `ChangeCriticalRegionHoopDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopDiameterM {
  mutation: "changeCriticalRegionHoopDiameterM";
  id: string;
  newHoopDiameterM: number;
}

export const parseChangeCriticalRegionHoopDiameterM: NormWireReader<ChangeCriticalRegionHoopDiameterM> = normWireObject<ChangeCriticalRegionHoopDiameterM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopDiameterM")), id: normWireRequired(normWireString), newHoopDiameterM: normWireRequired(normWireNumber) });
