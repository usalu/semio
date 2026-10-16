/** ↔️ `change-member-buckling-length-z` wire twin: the leaf payload `ChangeMemberBucklingLengthZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLengthZ {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBucklingLengthZ: NormWireReader<ChangeMemberBucklingLengthZ> = normWireObject<ChangeMemberBucklingLengthZ>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
