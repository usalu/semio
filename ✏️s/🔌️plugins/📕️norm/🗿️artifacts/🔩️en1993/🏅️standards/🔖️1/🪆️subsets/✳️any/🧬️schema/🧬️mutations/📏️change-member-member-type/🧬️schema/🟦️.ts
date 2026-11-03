/** 📏️ `change-member-member-type` wire twin: the leaf payload `ChangeMemberMemberType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelMemberType, parseSteelMemberType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberMemberType {
  id: string;
  newMemberType: SteelMemberType;
}

export const parseChangeMemberMemberType: NormWireReader<ChangeMemberMemberType> = normWireObject<ChangeMemberMemberType>({ id: normWireRequired(normWireString), newMemberType: normWireRequired(parseSteelMemberType) });
