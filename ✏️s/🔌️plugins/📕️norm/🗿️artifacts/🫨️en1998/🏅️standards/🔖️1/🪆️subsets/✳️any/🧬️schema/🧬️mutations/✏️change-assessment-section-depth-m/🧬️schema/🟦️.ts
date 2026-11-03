/** ✏️ `change-assessment-section-depth-m` wire twin: the leaf payload `ChangeAssessmentSectionDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentSectionDepthM {
  mutation: "changeAssessmentSectionDepthM";
  id: string;
  newSectionDepthM: number;
}

export const parseChangeAssessmentSectionDepthM: NormWireReader<ChangeAssessmentSectionDepthM> = normWireObject<ChangeAssessmentSectionDepthM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentSectionDepthM")), id: normWireRequired(normWireString), newSectionDepthM: normWireRequired(normWireNumber) });
