/** 📏️ `change-member-section-id` wire twin: the leaf payload `ChangeMemberSectionId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSectionId {
  id: string;
  newSectionId: string;
}

export const parseChangeMemberSectionId: NormWireReader<ChangeMemberSectionId> = normWireObject<ChangeMemberSectionId>({ id: normWireRequired(normWireString), newSectionId: normWireRequired(normWireString) });
