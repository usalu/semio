/** ➕️ `insert-member-action` wire twin: the leaf payload `InsertMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CharacteristicAction, parseCharacteristicAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMemberAction {
  memberId: string;
  index: number;
  action: CharacteristicAction;
}

export const parseInsertMemberAction: NormWireReader<InsertMemberAction> = normWireObject<InsertMemberAction>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseCharacteristicAction) });
