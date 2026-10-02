/** ➖️ `remove-assessment` wire twin: the leaf payload `RemoveAssessment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveAssessment {
  mutation: "removeAssessment";
  index: number;
}

export const parseRemoveAssessment: NormWireReader<RemoveAssessment> = normWireObject<RemoveAssessment>({ mutation: normWireRequired(normWireLiteral("removeAssessment")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
