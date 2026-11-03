/** 🔁️ `change-fatigue-stress-history` wire twin: the leaf payload `ChangeFatigueStressHistory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressHistory {
  id: string;
  newStressHistory: string;
}

export const parseChangeFatigueStressHistory: NormWireReader<ChangeFatigueStressHistory> = normWireObject<ChangeFatigueStressHistory>({ id: normWireRequired(normWireString), newStressHistory: normWireRequired(normWireString) });
