/** 📇️ `change-case-entry-id` wire twin: the leaf payload `ChangeCaseEntryId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseEntryId {
  mutation: "changeCaseEntryId";
  caseId: string;
  newValue: string;
}

export const parseChangeCaseEntryId: NormWireReader<ChangeCaseEntryId> = normWireObject<ChangeCaseEntryId>({ mutation: normWireRequired(normWireLiteral("changeCaseEntryId")), caseId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
