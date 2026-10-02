/** ↔️ `change-member-bearing-length` wire twin: the leaf payload `ChangeMemberBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBearingLength {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBearingLength: NormWireReader<ChangeMemberBearingLength> = normWireObject<ChangeMemberBearingLength>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
