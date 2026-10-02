/** 📤 `remove-variable` wire twin: the leaf payload `RemoveVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveVariable {
  mutation: "removeVariable";
  index: number;
}

export const parseRemoveVariable: NormWireReader<RemoveVariable> = normWireObject<RemoveVariable>({ mutation: normWireRequired(normWireLiteral("removeVariable")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
