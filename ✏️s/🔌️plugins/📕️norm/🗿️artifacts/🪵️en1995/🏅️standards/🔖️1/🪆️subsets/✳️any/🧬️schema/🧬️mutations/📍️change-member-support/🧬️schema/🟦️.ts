/** 📍️ `change-member-support` wire twin: the leaf payload `ChangeMemberSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SupportType, parseSupportType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberSupport {
  memberId: string;
  newValue: SupportType;
}

export const parseChangeMemberSupport: NormWireReader<ChangeMemberSupport> = normWireObject<ChangeMemberSupport>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(parseSupportType) });
