/** ⬇️ `change-member-action-q-line` wire twin: the leaf payload `ChangeMemberActionQLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionQLine {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionQLine: NormWireReader<ChangeMemberActionQLine> = normWireObject<ChangeMemberActionQLine>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
