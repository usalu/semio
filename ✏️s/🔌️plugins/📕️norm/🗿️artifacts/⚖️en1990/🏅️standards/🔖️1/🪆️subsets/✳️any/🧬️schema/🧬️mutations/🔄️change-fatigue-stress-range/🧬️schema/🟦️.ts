/** 🔄 `change-fatigue-stress-range` wire twin: the leaf payload `ChangeFatigueStressRange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressRange {
  mutation: "changeFatigueStressRange";
  id: string;
  newStressRange: number;
}

export const parseChangeFatigueStressRange: NormWireReader<ChangeFatigueStressRange> = normWireObject<ChangeFatigueStressRange>({ mutation: normWireRequired(normWireLiteral("changeFatigueStressRange")), id: normWireRequired(normWireString), newStressRange: normWireRequired(normWireNumber) });
