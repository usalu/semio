/** ↔️ `change-member-bearing-length` wire twin: the leaf payload `ChangeMemberBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBearingLength {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBearingLength: NormWireReader<ChangeMemberBearingLength> = normWireObject<ChangeMemberBearingLength>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
