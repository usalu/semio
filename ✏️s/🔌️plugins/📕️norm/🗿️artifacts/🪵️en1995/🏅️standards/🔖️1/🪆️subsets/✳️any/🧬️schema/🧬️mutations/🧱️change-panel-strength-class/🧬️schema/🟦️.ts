/** 🧱️ `change-panel-strength-class` wire twin: the leaf payload `ChangePanelStrengthClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StrengthClass, parseStrengthClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePanelStrengthClass {
  panelId: string;
  newValue: StrengthClass;
}

export const parseChangePanelStrengthClass: NormWireReader<ChangePanelStrengthClass> = normWireObject<ChangePanelStrengthClass>({ panelId: normWireRequired(normWireString), newValue: normWireRequired(parseStrengthClass) });
