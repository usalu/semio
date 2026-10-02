/** ↕️ `change-member-h` wire twin: the leaf payload `ChangeMemberH`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberH {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberH: NormWireReader<ChangeMemberH> = normWireObject<ChangeMemberH>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
