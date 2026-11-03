/** ✏️ `change-assessment-diag-reinf-ratio` wire twin: the leaf payload `ChangeAssessmentDiagReinfRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentDiagReinfRatio {
  mutation: "changeAssessmentDiagReinfRatio";
  id: string;
  newDiagonalReinforcementRatio: number;
}

export const parseChangeAssessmentDiagReinfRatio: NormWireReader<ChangeAssessmentDiagReinfRatio> = normWireObject<ChangeAssessmentDiagReinfRatio>({ mutation: normWireRequired(normWireLiteral("changeAssessmentDiagReinfRatio")), id: normWireRequired(normWireString), newDiagonalReinforcementRatio: normWireRequired(normWireNumber) });
