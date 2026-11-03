/** 🏗️ `change-member-material-id` wire twin: the leaf payload `ChangeMemberMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMaterialId {
  mutation: "changeMemberMaterialId";
  memberId: string;
  newMaterialId: string;
}

export const parseChangeMemberMaterialId: NormWireReader<ChangeMemberMaterialId> = normWireObject<ChangeMemberMaterialId>({ mutation: normWireRequired(normWireLiteral("changeMemberMaterialId")), memberId: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
