/** ✏️ `change-assessment-core-width-m` wire twin: the leaf payload `ChangeAssessmentCoreWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentCoreWidthM {
  mutation: "changeAssessmentCoreWidthM";
  id: string;
  newCoreWidthM: number;
}

export const parseChangeAssessmentCoreWidthM: NormWireReader<ChangeAssessmentCoreWidthM> = normWireObject<ChangeAssessmentCoreWidthM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentCoreWidthM")), id: normWireRequired(normWireString), newCoreWidthM: normWireRequired(normWireNumber) });
