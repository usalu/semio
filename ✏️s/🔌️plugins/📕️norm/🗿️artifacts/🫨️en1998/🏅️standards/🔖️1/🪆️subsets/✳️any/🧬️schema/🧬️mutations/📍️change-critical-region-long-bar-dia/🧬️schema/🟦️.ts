/** 📍️ `change-critical-region-long-bar-dia` wire twin: the leaf payload `ChangeCriticalRegionLongitudinalBarDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionLongitudinalBarDiameterM {
  mutation: "changeCriticalRegionLongitudinalBarDiameterM";
  id: string;
  newLongitudinalBarDiameterM: number;
}

export const parseChangeCriticalRegionLongitudinalBarDiameterM: NormWireReader<ChangeCriticalRegionLongitudinalBarDiameterM> = normWireObject<ChangeCriticalRegionLongitudinalBarDiameterM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionLongitudinalBarDiameterM")), id: normWireRequired(normWireString), newLongitudinalBarDiameterM: normWireRequired(normWireNumber) });
