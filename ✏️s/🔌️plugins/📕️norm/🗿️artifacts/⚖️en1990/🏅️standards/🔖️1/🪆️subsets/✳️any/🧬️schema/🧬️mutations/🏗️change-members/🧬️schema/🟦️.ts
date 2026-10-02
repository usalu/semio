/** 🏗️ `change-members` wire twin: the leaf payload `ChangeMembers`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990Member, parseEn1990Member } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMembers {
  mutation: "changeMembers";
  newMembers: En1990Member[];
}

export const parseChangeMembers: NormWireReader<ChangeMembers> = normWireObject<ChangeMembers>({ mutation: normWireRequired(normWireLiteral("changeMembers")), newMembers: normWireRequired(normWireArray(parseEn1990Member)) });
