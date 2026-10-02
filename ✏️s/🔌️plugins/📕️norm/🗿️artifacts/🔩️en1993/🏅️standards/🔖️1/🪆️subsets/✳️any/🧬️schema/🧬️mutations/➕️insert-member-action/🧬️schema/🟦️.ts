/** ➕️ `insert-member-action` wire twin: the leaf payload `InsertMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type MemberAction, parseMemberAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMemberAction {
  index: number;
  memberAction: MemberAction;
}

export const parseInsertMemberAction: NormWireReader<InsertMemberAction> = normWireObject<InsertMemberAction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), memberAction: normWireRequired(parseMemberAction) });
