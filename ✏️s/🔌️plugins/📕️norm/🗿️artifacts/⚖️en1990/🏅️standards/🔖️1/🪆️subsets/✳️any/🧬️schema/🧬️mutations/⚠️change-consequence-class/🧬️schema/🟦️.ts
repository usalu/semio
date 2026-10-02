/** ⚠️ `change-consequence-class` wire twin: the leaf payload `ChangeConsequenceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConsequenceClass {
  mutation: "changeConsequenceClass";
  newConsequenceClass: number;
}

export const parseChangeConsequenceClass: NormWireReader<ChangeConsequenceClass> = normWireObject<ChangeConsequenceClass>({ mutation: normWireRequired(normWireLiteral("changeConsequenceClass")), newConsequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
