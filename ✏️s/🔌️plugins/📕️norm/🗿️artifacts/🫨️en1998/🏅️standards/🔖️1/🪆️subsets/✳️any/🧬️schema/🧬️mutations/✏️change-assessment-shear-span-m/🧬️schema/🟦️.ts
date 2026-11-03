/** ✏️ `change-assessment-shear-span-m` wire twin: the leaf payload `ChangeAssessmentShearSpanM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentShearSpanM {
  mutation: "changeAssessmentShearSpanM";
  id: string;
  newShearSpanM: number;
}

export const parseChangeAssessmentShearSpanM: NormWireReader<ChangeAssessmentShearSpanM> = normWireObject<ChangeAssessmentShearSpanM>({ mutation: normWireRequired(normWireLiteral("changeAssessmentShearSpanM")), id: normWireRequired(normWireString), newShearSpanM: normWireRequired(normWireNumber) });
