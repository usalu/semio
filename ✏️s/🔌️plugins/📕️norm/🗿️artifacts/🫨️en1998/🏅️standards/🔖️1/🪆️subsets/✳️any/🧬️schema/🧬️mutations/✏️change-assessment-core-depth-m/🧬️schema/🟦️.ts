/** ✏️ `change-assessment-core-depth-m` wire twin: the leaf payload `ChangeAssessmentCoreDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentCoreDepthM {
  mutation: "changeAssessmentCoreDepthM";
  id: string;
  newCoreDepthM: number;
}

export const parseChangeAssessmentCoreDepthM: NormWireReader<ChangeAssessmentCoreDepthM> = normWireObject<ChangeAssessmentCoreDepthM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentCoreDepthM")), id: normWireRequired(normWireString), newCoreDepthM: normWireRequired(normWireNumber) });
