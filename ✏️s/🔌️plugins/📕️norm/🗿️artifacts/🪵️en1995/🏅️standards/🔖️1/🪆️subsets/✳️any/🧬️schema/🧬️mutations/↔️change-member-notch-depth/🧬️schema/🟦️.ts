/** ↔️ `change-member-notch-depth` wire twin: the leaf payload `ChangeMemberNotchDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberNotchDepth {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberNotchDepth: NormWireReader<ChangeMemberNotchDepth> = normWireObject<ChangeMemberNotchDepth>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
