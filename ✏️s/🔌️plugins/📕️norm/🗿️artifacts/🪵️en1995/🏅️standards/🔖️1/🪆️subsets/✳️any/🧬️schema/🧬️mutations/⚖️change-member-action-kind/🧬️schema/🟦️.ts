/** ⚖️ `change-member-action-kind` wire twin: the leaf payload `ChangeMemberActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionKind {
  memberId: string;
  actionId: string;
  newValue: string;
}

export const parseChangeMemberActionKind: NormWireReader<ChangeMemberActionKind> = normWireObject<ChangeMemberActionKind>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
