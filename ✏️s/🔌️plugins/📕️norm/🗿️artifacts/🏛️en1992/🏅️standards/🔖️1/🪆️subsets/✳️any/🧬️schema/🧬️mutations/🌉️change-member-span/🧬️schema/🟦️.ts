/** 🌉️ `change-member-span` wire twin: the leaf payload `ChangeMemberSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberSpan {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberSpan: NormWireReader<ChangeMemberSpan> = normWireObject<ChangeMemberSpan>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
