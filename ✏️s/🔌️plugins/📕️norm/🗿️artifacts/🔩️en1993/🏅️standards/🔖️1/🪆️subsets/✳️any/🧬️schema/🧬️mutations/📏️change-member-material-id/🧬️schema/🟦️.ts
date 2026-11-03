/** 📏️ `change-member-material-id` wire twin: the leaf payload `ChangeMemberMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMaterialId {
  id: string;
  newMaterialId: string;
}

export const parseChangeMemberMaterialId: NormWireReader<ChangeMemberMaterialId> = normWireObject<ChangeMemberMaterialId>({ id: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
