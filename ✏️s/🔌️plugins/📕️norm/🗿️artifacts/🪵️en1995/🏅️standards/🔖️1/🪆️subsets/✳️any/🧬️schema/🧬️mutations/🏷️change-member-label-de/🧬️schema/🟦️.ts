/** 🏷️ `change-member-label-de` wire twin: the leaf payload `ChangeMemberLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberLabelDe {
  memberId: string;
  newValue: string;
}

export const parseChangeMemberLabelDe: NormWireReader<ChangeMemberLabelDe> = normWireObject<ChangeMemberLabelDe>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
