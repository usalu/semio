/** 📍️ `change-critical-region-hoop-spacing-m` wire twin: the leaf payload `ChangeCriticalRegionHoopSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopSpacingM {
  mutation: "changeCriticalRegionHoopSpacingM";
  id: string;
  newHoopSpacingM: number;
}

export const parseChangeCriticalRegionHoopSpacingM: NormWireReader<ChangeCriticalRegionHoopSpacingM> = normWireObject<ChangeCriticalRegionHoopSpacingM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopSpacingM")), id: normWireRequired(normWireString), newHoopSpacingM: normWireRequired(normWireNumber) });
