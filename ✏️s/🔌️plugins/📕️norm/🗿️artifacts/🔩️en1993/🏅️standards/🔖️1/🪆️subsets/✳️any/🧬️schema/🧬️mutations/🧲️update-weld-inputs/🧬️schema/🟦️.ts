/** 🧲️ `update-weld-inputs` wire twin: the leaf payload `UpdateWeldInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type MemberAction, parseMemberAction } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateWeldInputs {
  memberAction: MemberAction;
}

export const parseUpdateWeldInputs: NormWireReader<UpdateWeldInputs> = normWireObject<UpdateWeldInputs>({ memberAction: normWireRequired(parseMemberAction) });
