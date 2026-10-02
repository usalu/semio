/** 📍️ `change-member-support` wire twin: the leaf payload `ChangeMemberSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSupportType, type SupportType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberSupport {
  memberId: string;
  newValue: SupportType;
}

export const parseChangeMemberSupport: NormWireReader<ChangeMemberSupport> = normWireObject<ChangeMemberSupport>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(parseSupportType) });
