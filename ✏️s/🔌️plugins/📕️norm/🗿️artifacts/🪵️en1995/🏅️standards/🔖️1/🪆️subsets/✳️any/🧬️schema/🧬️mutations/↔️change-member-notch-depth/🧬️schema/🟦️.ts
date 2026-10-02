/** ↔️ `change-member-notch-depth` wire twin: the leaf payload `ChangeMemberNotchDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberNotchDepth {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberNotchDepth: NormWireReader<ChangeMemberNotchDepth> = normWireObject<ChangeMemberNotchDepth>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
