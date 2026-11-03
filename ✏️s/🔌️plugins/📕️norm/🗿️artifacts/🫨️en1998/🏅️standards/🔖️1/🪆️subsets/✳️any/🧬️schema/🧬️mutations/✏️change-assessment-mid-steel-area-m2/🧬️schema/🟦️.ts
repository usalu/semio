/** ✏️ `change-assessment-mid-steel-area-m2` wire twin: the leaf payload `ChangeAssessmentMidSteelAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentMidSteelAreaM2 {
  mutation: "changeAssessmentMidSteelAreaM2";
  id: string;
  newIntermediateSteelAreaM2: number;
}

export const parseChangeAssessmentMidSteelAreaM2: NormWireReader<ChangeAssessmentMidSteelAreaM2> = normWireObject<ChangeAssessmentMidSteelAreaM2>({ mutation: normWireRequired(normWireLiteral("changeAssessmentMidSteelAreaM2")), id: normWireRequired(normWireString), newIntermediateSteelAreaM2: normWireRequired(normWireNumber) });
