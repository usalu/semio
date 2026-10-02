/** 🏷️ `change-member-label-en` wire twin: the leaf payload `ChangeMemberLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberLabelEn {
  memberId: string;
  newValue: string;
}

export const parseChangeMemberLabelEn: NormWireReader<ChangeMemberLabelEn> = normWireObject<ChangeMemberLabelEn>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
