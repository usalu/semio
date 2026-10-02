/** 🧮 `change-method` wire twin: the leaf payload `ChangeMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599CalculationMethod, parseDin18599CalculationMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMethod {
  mutation: "changeMethod";
  newMethod: Din18599CalculationMethod;
}

export const parseChangeMethod: NormWireReader<ChangeMethod> = normWireObject<ChangeMethod>({ mutation: normWireRequired(normWireLiteral("changeMethod")), newMethod: normWireRequired(parseDin18599CalculationMethod) });
