/** 🏋️ `change-action-load-case-id` wire twin: the leaf payload `ChangeActionLoadCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionLoadCaseId {
  id: string;
  newLoadCaseId: string;
}

export const parseChangeActionLoadCaseId: NormWireReader<ChangeActionLoadCaseId> = normWireObject<ChangeActionLoadCaseId>({ id: normWireRequired(normWireString), newLoadCaseId: normWireRequired(normWireString) });
