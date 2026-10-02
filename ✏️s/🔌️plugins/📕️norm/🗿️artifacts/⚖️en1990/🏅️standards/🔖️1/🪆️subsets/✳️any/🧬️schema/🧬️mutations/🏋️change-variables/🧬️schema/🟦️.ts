/** 🏋️ `change-variables` wire twin: the leaf payload `ChangeVariables`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990VariableAction, parseEn1990VariableAction } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeVariables {
  mutation: "changeVariables";
  newVariables: En1990VariableAction[];
}

export const parseChangeVariables: NormWireReader<ChangeVariables> = normWireObject<ChangeVariables>({ mutation: normWireRequired(normWireLiteral("changeVariables")), newVariables: normWireRequired(normWireArray(parseEn1990VariableAction)) });
