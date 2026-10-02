/** ↔️ `change-member-notch-distance` wire twin: the leaf payload `ChangeMemberNotchDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberNotchDistance {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberNotchDistance: NormWireReader<ChangeMemberNotchDistance> = normWireObject<ChangeMemberNotchDistance>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
