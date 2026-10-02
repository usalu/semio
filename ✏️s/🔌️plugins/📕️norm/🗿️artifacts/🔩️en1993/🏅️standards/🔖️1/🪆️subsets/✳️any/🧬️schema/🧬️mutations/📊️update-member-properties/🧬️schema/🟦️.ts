/** 📊️ `update-member-properties` wire twin: the leaf payload `UpdateMemberProperties`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelMember, type SteelMember } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateMemberProperties {
  member: SteelMember;
}

export const parseUpdateMemberProperties: NormWireReader<UpdateMemberProperties> = normWireObject<UpdateMemberProperties>({ member: normWireRequired(parseSteelMember) });
