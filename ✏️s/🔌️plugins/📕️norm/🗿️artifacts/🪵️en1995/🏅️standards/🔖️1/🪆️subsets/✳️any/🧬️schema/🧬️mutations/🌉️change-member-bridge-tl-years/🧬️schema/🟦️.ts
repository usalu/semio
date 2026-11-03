/** 🌉️ `change-member-bridge-tl-years` wire twin: the leaf payload `ChangeMemberBridgeTLYears`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBridgeTLYears {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeTLYears: NormWireReader<ChangeMemberBridgeTLYears> = normWireObject<ChangeMemberBridgeTLYears>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
