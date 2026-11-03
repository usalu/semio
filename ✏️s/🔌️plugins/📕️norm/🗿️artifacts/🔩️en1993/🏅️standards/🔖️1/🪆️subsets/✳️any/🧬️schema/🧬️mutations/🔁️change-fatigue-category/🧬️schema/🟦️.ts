/** 🔁️ `change-fatigue-category` wire twin: the leaf payload `ChangeFatigueCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueCategory {
  id: string;
  newCategory: number;
}

export const parseChangeFatigueCategory: NormWireReader<ChangeFatigueCategory> = normWireObject<ChangeFatigueCategory>({ id: normWireRequired(normWireString), newCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
