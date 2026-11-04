/** ✏️ `change-assessment-analysis-method` wire twin: the leaf payload `ChangeAssessmentAnalysisMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AnalysisMethodChoice, parseAnalysisMethodChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAssessmentAnalysisMethod {
  mutation: "changeAssessmentAnalysisMethod";
  id: string;
  newAnalysisMethodChoice: AnalysisMethodChoice;
}

export const parseChangeAssessmentAnalysisMethod: NormWireReader<ChangeAssessmentAnalysisMethod> = normWireObject<ChangeAssessmentAnalysisMethod>({ mutation: normWireRequired(normWireLiteral("changeAssessmentAnalysisMethod")), id: normWireRequired(normWireString), newAnalysisMethodChoice: normWireRequired(parseAnalysisMethodChoice) });
