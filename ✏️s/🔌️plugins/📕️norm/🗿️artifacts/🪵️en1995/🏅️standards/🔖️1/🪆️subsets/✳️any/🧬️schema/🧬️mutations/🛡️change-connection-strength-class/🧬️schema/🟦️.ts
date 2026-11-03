/** 🛡️ `change-connection-strength-class` wire twin: the leaf payload `ChangeConnectionStrengthClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StrengthClass, parseStrengthClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnectionStrengthClass {
  connectionId: string;
  newValue: StrengthClass;
}

export const parseChangeConnectionStrengthClass: NormWireReader<ChangeConnectionStrengthClass> = normWireObject<ChangeConnectionStrengthClass>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(parseStrengthClass) });
