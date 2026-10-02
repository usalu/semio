/** ➕ `add-member` wire twin: the leaf payload `AddMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumMember, parseAluminiumMember } from "../../../📸️snapshot/🟦️.ts";

export interface AddMember {
  mutation: "addMember";
  index: number;
  member: AluminiumMember;
}

export const parseAddMember: NormWireReader<AddMember> = normWireObject<AddMember>({ mutation: normWireRequired(normWireLiteral("addMember")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), member: normWireRequired(parseAluminiumMember) });
