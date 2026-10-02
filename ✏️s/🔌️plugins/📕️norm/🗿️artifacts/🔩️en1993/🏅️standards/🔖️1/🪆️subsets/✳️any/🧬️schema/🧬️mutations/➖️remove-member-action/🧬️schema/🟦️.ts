/** ➖️ `remove-member-action` wire twin: the leaf payload `RemoveMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveMemberAction {
  index: number;
}

export const parseRemoveMemberAction: NormWireReader<RemoveMemberAction> = normWireObject<RemoveMemberAction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
