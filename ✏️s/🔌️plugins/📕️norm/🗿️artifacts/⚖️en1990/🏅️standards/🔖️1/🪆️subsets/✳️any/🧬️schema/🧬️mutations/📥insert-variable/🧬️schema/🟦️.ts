/** 📥 `insert-variable` wire twin: the leaf payload `InsertVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990VariableAction, parseEn1990VariableAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertVariable {
  mutation: "insertVariable";
  index: number;
  item: En1990VariableAction;
}

export const parseInsertVariable: NormWireReader<InsertVariable> = normWireObject<InsertVariable>({ mutation: normWireRequired(normWireLiteral("insertVariable")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990VariableAction) });
