/** 🧱️ `change-frame-action-load-case-id` wire twin: the leaf payload `ChangeFrameActionLoadCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionLoadCaseId {
  id: string;
  newLoadCaseId: string;
}

export const parseChangeFrameActionLoadCaseId: NormWireReader<ChangeFrameActionLoadCaseId> = normWireObject<ChangeFrameActionLoadCaseId>({ id: normWireRequired(normWireString), newLoadCaseId: normWireRequired(normWireString) });
