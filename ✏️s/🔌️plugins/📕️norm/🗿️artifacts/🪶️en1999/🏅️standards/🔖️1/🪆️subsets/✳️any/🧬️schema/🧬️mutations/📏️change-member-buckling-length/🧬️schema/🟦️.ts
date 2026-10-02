/** 📏️ `change-member-buckling-length` wire twin: the leaf payload `ChangeMemberBucklingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBucklingLength {
  mutation: "changeMemberBucklingLength";
  memberId: string;
  axis: string;
  newLength: number;
}

export const parseChangeMemberBucklingLength: NormWireReader<ChangeMemberBucklingLength> = normWireObject<ChangeMemberBucklingLength>({ mutation: normWireRequired(normWireLiteral("changeMemberBucklingLength")), memberId: normWireRequired(normWireString), axis: normWireRequired(normWireString), newLength: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
