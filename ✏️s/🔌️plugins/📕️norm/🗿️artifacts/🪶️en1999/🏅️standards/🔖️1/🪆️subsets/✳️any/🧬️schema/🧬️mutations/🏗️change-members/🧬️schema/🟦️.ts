/** 🏗️ `change-members` wire twin: the leaf payload `ChangeMembers`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumMember, parseAluminiumMember } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMembers {
  mutation: "changeMembers";
  members: AluminiumMember[];
}

export const parseChangeMembers: NormWireReader<ChangeMembers> = normWireObject<ChangeMembers>({ mutation: normWireRequired(normWireLiteral("changeMembers")), members: normWireRequired(normWireArray(parseAluminiumMember)) });
