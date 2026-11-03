/** 🛡️ `change-member-strength-class` wire twin: the leaf payload `ChangeMemberStrengthClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StrengthClass, parseStrengthClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberStrengthClass {
  memberId: string;
  newValue: StrengthClass;
}

export const parseChangeMemberStrengthClass: NormWireReader<ChangeMemberStrengthClass> = normWireObject<ChangeMemberStrengthClass>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(parseStrengthClass) });
