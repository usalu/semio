/** 🔄️ `change-fatigue-m1` wire twin: the leaf payload `ChangeFatigueM1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueM1 {
  mutation: "changeFatigueM1";
  fatigueDetailId: string;
  newM1: number;
}

export const parseChangeFatigueM1: NormWireReader<ChangeFatigueM1> = normWireObject<ChangeFatigueM1>({ mutation: normWireRequired(normWireLiteral("changeFatigueM1")), fatigueDetailId: normWireRequired(normWireString), newM1: normWireRequired(normWireNumber) });
