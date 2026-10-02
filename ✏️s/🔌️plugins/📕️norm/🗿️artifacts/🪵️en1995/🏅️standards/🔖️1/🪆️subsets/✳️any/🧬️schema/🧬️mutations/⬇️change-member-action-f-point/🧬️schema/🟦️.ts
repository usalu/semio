/** ⬇️ `change-member-action-f-point` wire twin: the leaf payload `ChangeMemberActionFPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionFPoint {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionFPoint: NormWireReader<ChangeMemberActionFPoint> = normWireObject<ChangeMemberActionFPoint>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
