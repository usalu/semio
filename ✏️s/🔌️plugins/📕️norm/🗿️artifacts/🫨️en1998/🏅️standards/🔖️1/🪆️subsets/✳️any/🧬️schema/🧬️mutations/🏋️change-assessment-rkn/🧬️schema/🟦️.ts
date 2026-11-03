/** 🏋️ `change-assessment-rkn` wire twin: the leaf payload `ChangeAssessmentRKN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentRKN {
  mutation: "changeAssessmentRKN";
  id: string;
  newRKN: number;
}

export const parseChangeAssessmentRKN: NormWireReader<ChangeAssessmentRKN> = normWireObject<ChangeAssessmentRKN>({ mutation: normWireRequired(normWireLiteral("changeAssessmentRKN")), id: normWireRequired(normWireString), newRKN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
