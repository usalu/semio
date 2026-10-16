/** ↔️ `change-member-buckling-length-y` wire twin: the leaf payload `ChangeMemberBucklingLengthY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLengthY {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBucklingLengthY: NormWireReader<ChangeMemberBucklingLengthY> = normWireObject<ChangeMemberBucklingLengthY>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
