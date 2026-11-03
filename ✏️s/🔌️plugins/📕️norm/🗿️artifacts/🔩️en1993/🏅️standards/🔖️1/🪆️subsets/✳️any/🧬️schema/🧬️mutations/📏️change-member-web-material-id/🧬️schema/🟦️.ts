/** 📏️ `change-member-web-material-id` wire twin: the leaf payload `ChangeMemberWebMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWebMaterialId {
  id: string;
  newWebMaterialId: string;
}

export const parseChangeMemberWebMaterialId: NormWireReader<ChangeMemberWebMaterialId> = normWireObject<ChangeMemberWebMaterialId>({ id: normWireRequired(normWireString), newWebMaterialId: normWireRequired(normWireString) });
