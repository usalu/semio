/** ⬇️ `change-member-action-f-point` wire twin: the leaf payload `ChangeMemberActionFPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionFPoint {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionFPoint: NormWireReader<ChangeMemberActionFPoint> = normWireObject<ChangeMemberActionFPoint>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
