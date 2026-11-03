/** ✏️ `change-assessment-section-width-m` wire twin: the leaf payload `ChangeAssessmentSectionWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentSectionWidthM {
  mutation: "changeAssessmentSectionWidthM";
  id: string;
  newSectionWidthM: number;
}

export const parseChangeAssessmentSectionWidthM: NormWireReader<ChangeAssessmentSectionWidthM> = normWireObject<ChangeAssessmentSectionWidthM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentSectionWidthM")), id: normWireRequired(normWireString), newSectionWidthM: normWireRequired(normWireNumber) });
