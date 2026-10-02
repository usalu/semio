/** 🏋️ `change-assessment-rkn` wire twin: the leaf payload `ChangeAssessmentRKN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssessmentRKN {
  mutation: "changeAssessmentRKN";
  index: number;
  newRKN: number;
}

export const parseChangeAssessmentRKN: NormWireReader<ChangeAssessmentRKN> = normWireObject<ChangeAssessmentRKN>({ mutation: normWireRequired(normWireLiteral("changeAssessmentRKN")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newRKN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
