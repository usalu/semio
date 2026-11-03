/** ✏️ `change-assessment-rbar-spacing-m` wire twin: the leaf payload `ChangeAssessmentRbarSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentRbarSpacingM {
  mutation: "changeAssessmentRbarSpacingM";
  id: string;
  newRestrainedBarSpacingM: number;
}

export const parseChangeAssessmentRbarSpacingM: NormWireReader<ChangeAssessmentRbarSpacingM> = normWireObject<ChangeAssessmentRbarSpacingM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentRbarSpacingM")), id: normWireRequired(normWireString), newRestrainedBarSpacingM: normWireRequired(normWireNumber) });
