/** 🪵️ `change-pile-action-load-case-id` wire twin: the leaf payload `ChangePileActionLoadCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileActionLoadCaseId {
  id: string;
  elementId: string;
  newLoadCaseId: string;
}

export const parseChangePileActionLoadCaseId: NormWireReader<ChangePileActionLoadCaseId> = normWireObject<ChangePileActionLoadCaseId>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newLoadCaseId: normWireRequired(normWireString) });
