/** 🥶️ `change-cf-action-load-case-id` wire twin: the leaf payload `ChangeCfActionLoadCaseId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfActionLoadCaseId {
  id: string;
  elementId: string;
  newLoadCaseId: string;
}

export const parseChangeCfActionLoadCaseId: NormWireReader<ChangeCfActionLoadCaseId> = normWireObject<ChangeCfActionLoadCaseId>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newLoadCaseId: normWireRequired(normWireString) });
