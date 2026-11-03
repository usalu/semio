/** 🧱️ `change-fatigue-stress-detail-id` wire twin: the leaf payload `ChangeFatigueStressDetailId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressDetailId {
  id: string;
  newDetailId: string;
}

export const parseChangeFatigueStressDetailId: NormWireReader<ChangeFatigueStressDetailId> = normWireObject<ChangeFatigueStressDetailId>({ id: normWireRequired(normWireString), newDetailId: normWireRequired(normWireString) });
