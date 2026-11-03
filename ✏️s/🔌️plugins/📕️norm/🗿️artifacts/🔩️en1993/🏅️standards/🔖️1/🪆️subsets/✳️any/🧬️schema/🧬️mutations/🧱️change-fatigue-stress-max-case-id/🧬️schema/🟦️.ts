/** 🧱️ `change-fatigue-stress-max-case-id` wire twin: the leaf payload `ChangeFatigueStressMaxCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressMaxCaseId {
  id: string;
  newMaxCaseId: string;
}

export const parseChangeFatigueStressMaxCaseId: NormWireReader<ChangeFatigueStressMaxCaseId> = normWireObject<ChangeFatigueStressMaxCaseId>({ id: normWireRequired(normWireString), newMaxCaseId: normWireRequired(normWireString) });
