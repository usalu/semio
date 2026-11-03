/** ✏️ `change-assessment-rbar-count` wire twin: the leaf payload `ChangeAssessmentRbarCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentRbarCount {
  mutation: "changeAssessmentRbarCount";
  id: string;
  newRestrainedBarCount: number;
}

export const parseChangeAssessmentRbarCount: NormWireReader<ChangeAssessmentRbarCount> = normWireObject<ChangeAssessmentRbarCount>({ mutation: normWireRequired(normWireLiteral("changeAssessmentRbarCount")), id: normWireRequired(normWireString), newRestrainedBarCount: normWireRequired(normWireNumber) });
