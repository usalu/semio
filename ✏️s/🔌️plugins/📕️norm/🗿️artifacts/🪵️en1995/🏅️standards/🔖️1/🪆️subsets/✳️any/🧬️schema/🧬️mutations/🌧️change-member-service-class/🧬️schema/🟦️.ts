/** 🌧️ `change-member-service-class` wire twin: the leaf payload `ChangeMemberServiceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberServiceClass {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberServiceClass: NormWireReader<ChangeMemberServiceClass> = normWireObject<ChangeMemberServiceClass>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":3})) });
