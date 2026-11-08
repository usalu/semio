/** 📍️ `change-critical-region-long-steel-area` wire twin: the leaf payload `ChangeCriticalRegionLongitudinalSteelAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionLongitudinalSteelAreaM2 {
  mutation: "changeCriticalRegionLongitudinalSteelAreaM2";
  id: string;
  newLongitudinalSteelAreaM2: number;
}

export const parseChangeCriticalRegionLongitudinalSteelAreaM2: NormWireReader<ChangeCriticalRegionLongitudinalSteelAreaM2> = normWireObject<ChangeCriticalRegionLongitudinalSteelAreaM2>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionLongitudinalSteelAreaM2")), id: normWireRequired(normWireString), newLongitudinalSteelAreaM2: normWireRequired(normWireNumber) });
