/** ✏️ `change-assessment-as2-area-m2` wire twin: the leaf payload `ChangeAssessmentAs2AreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentAs2AreaM2 {
  mutation: "changeAssessmentAs2AreaM2";
  id: string;
  newCompressionSteelAreaM2: number;
}

export const parseChangeAssessmentAs2AreaM2: NormWireReader<ChangeAssessmentAs2AreaM2> = normWireObject<ChangeAssessmentAs2AreaM2>({ mutation: normWireRequired(normWireLiteral("changeAssessmentAs2AreaM2")), id: normWireRequired(normWireString), newCompressionSteelAreaM2: normWireRequired(normWireNumber) });
