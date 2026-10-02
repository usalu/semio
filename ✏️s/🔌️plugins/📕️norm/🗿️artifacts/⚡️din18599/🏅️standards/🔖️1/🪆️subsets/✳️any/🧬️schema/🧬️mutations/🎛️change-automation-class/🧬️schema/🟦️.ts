/** 🎛️ `change-automation-class` wire twin: the leaf payload `ChangeAutomationClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599AutomationClass, parseDin18599AutomationClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAutomationClass {
  mutation: "changeAutomationClass";
  newAutomationClass: Din18599AutomationClass;
}

export const parseChangeAutomationClass: NormWireReader<ChangeAutomationClass> = normWireObject<ChangeAutomationClass>({ mutation: normWireRequired(normWireLiteral("changeAutomationClass")), newAutomationClass: normWireRequired(parseDin18599AutomationClass) });
