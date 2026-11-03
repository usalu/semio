/** 🔄️ `change-fatigue-m2` wire twin: the leaf payload `ChangeFatigueM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueM2 {
  mutation: "changeFatigueM2";
  fatigueDetailId: string;
  newM2: number;
}

export const parseChangeFatigueM2: NormWireReader<ChangeFatigueM2> = normWireObject<ChangeFatigueM2>({ mutation: normWireRequired(normWireLiteral("changeFatigueM2")), fatigueDetailId: normWireRequired(normWireString), newM2: normWireRequired(normWireNumber) });
