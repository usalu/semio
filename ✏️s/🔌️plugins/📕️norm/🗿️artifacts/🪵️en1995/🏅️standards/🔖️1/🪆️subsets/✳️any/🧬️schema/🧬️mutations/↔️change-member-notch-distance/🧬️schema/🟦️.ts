/** ↔️ `change-member-notch-distance` wire twin: the leaf payload `ChangeMemberNotchDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberNotchDistance {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberNotchDistance: NormWireReader<ChangeMemberNotchDistance> = normWireObject<ChangeMemberNotchDistance>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
