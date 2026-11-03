/** ✏️ `change-assessment-strp-spacing-m` wire twin: the leaf payload `ChangeAssessmentStrpSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentStrpSpacingM {
  mutation: "changeAssessmentStrpSpacingM";
  id: string;
  newStirrupSpacingM: number;
}

export const parseChangeAssessmentStrpSpacingM: NormWireReader<ChangeAssessmentStrpSpacingM> = normWireObject<ChangeAssessmentStrpSpacingM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentStrpSpacingM")), id: normWireRequired(normWireString), newStirrupSpacingM: normWireRequired(normWireNumber) });
