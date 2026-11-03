/** ⬇️ `change-patch-subject-id` wire twin: the leaf payload `ChangePatchSubjectId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchSubjectId {
  mutation: "changePatchSubjectId";
  patchLoadId: string;
  newSubjectId: string;
}

export const parseChangePatchSubjectId: NormWireReader<ChangePatchSubjectId> = normWireObject<ChangePatchSubjectId>({ mutation: normWireRequired(normWireLiteral("changePatchSubjectId")), patchLoadId: normWireRequired(normWireString), newSubjectId: normWireRequired(normWireString) });
