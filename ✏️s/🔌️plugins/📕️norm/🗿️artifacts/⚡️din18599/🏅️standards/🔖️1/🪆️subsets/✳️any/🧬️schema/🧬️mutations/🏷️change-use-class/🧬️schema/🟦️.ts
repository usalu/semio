/** 🏷️ `change-use-class` wire twin: the leaf payload `ChangeUseClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599UseClass, parseDin18599UseClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUseClass {
  mutation: "changeUseClass";
  newUseClass: Din18599UseClass;
}

export const parseChangeUseClass: NormWireReader<ChangeUseClass> = normWireObject<ChangeUseClass>({ mutation: normWireRequired(normWireLiteral("changeUseClass")), newUseClass: normWireRequired(parseDin18599UseClass) });
