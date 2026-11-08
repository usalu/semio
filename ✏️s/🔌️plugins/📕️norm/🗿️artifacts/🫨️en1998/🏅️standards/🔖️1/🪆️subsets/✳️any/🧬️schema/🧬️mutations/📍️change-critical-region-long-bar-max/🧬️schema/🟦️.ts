/** 📍️ `change-critical-region-long-bar-max` wire twin: the leaf payload `ChangeCriticalRegionLongitudinalBarMaxDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionLongitudinalBarMaxDiameterM {
  mutation: "changeCriticalRegionLongitudinalBarMaxDiameterM";
  id: string;
  newLongitudinalBarMaxDiameterM: number;
}

export const parseChangeCriticalRegionLongitudinalBarMaxDiameterM: NormWireReader<ChangeCriticalRegionLongitudinalBarMaxDiameterM> = normWireObject<ChangeCriticalRegionLongitudinalBarMaxDiameterM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionLongitudinalBarMaxDiameterM")), id: normWireRequired(normWireString), newLongitudinalBarMaxDiameterM: normWireRequired(normWireNumber) });
