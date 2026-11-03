/** 🧱️ `change-fatigue-stress-min-case-id` wire twin: the leaf payload `ChangeFatigueStressMinCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressMinCaseId {
  id: string;
  newMinCaseId: string;
}

export const parseChangeFatigueStressMinCaseId: NormWireReader<ChangeFatigueStressMinCaseId> = normWireObject<ChangeFatigueStressMinCaseId>({ id: normWireRequired(normWireString), newMinCaseId: normWireRequired(normWireString) });
