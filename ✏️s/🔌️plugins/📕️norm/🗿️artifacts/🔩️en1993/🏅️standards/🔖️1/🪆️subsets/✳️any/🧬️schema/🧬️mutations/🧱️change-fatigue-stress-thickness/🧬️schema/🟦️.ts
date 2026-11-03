/** 🧱️ `change-fatigue-stress-thickness` wire twin: the leaf payload `ChangeFatigueStressThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressThickness {
  id: string;
  newThickness: number;
}

export const parseChangeFatigueStressThickness: NormWireReader<ChangeFatigueStressThickness> = normWireObject<ChangeFatigueStressThickness>({ id: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
