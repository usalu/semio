/** ✏️ `change-assessment-eff-depth-m` wire twin: the leaf payload `ChangeAssessmentEffDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentEffDepthM {
  mutation: "changeAssessmentEffDepthM";
  id: string;
  newEffectiveDepthM: number;
}

export const parseChangeAssessmentEffDepthM: NormWireReader<ChangeAssessmentEffDepthM> = normWireObject<ChangeAssessmentEffDepthM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentEffDepthM")), id: normWireRequired(normWireString), newEffectiveDepthM: normWireRequired(normWireNumber) });
