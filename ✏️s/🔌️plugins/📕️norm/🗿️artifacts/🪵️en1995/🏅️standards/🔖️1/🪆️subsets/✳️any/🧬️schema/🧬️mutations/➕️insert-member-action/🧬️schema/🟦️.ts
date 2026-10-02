/** ➕️ `insert-member-action` wire twin: the leaf payload `InsertMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CharacteristicAction, parseCharacteristicAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMemberAction {
  memberId: string;
  index: number;
  action: CharacteristicAction;
}

export const parseInsertMemberAction: NormWireReader<InsertMemberAction> = normWireObject<InsertMemberAction>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseCharacteristicAction) });
