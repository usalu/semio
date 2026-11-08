/** 📍️ `change-critical-region-restr-bar-space` wire twin: the leaf payload `ChangeCriticalRegionRestrainedBarSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionRestrainedBarSpacingM {
  mutation: "changeCriticalRegionRestrainedBarSpacingM";
  id: string;
  newRestrainedBarSpacingM: number;
}

export const parseChangeCriticalRegionRestrainedBarSpacingM: NormWireReader<ChangeCriticalRegionRestrainedBarSpacingM> = normWireObject<ChangeCriticalRegionRestrainedBarSpacingM>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionRestrainedBarSpacingM")), id: normWireRequired(normWireString), newRestrainedBarSpacingM: normWireRequired(normWireNumber) });
