/** 🪢️ `change-tension-action-load-case-id` wire twin: the leaf payload `ChangeTensionActionLoadCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionActionLoadCaseId {
  id: string;
  elementId: string;
  newLoadCaseId: string;
}

export const parseChangeTensionActionLoadCaseId: NormWireReader<ChangeTensionActionLoadCaseId> = normWireObject<ChangeTensionActionLoadCaseId>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newLoadCaseId: normWireRequired(normWireString) });
