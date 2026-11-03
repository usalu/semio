/** ✏️ `change-assessment-as1-area-m2` wire twin: the leaf payload `ChangeAssessmentAs1AreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentAs1AreaM2 {
  mutation: "changeAssessmentAs1AreaM2";
  id: string;
  newTensionSteelAreaM2: number;
}

export const parseChangeAssessmentAs1AreaM2: NormWireReader<ChangeAssessmentAs1AreaM2> = normWireObject<ChangeAssessmentAs1AreaM2>({ mutation: normWireRequired(normWireLiteral("changeAssessmentAs1AreaM2")), id: normWireRequired(normWireString), newTensionSteelAreaM2: normWireRequired(normWireNumber) });
