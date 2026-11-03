/** ✏️ `change-assessment-gamma-el` wire twin: the leaf payload `ChangeAssessmentGammaEl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentGammaEl {
  mutation: "changeAssessmentGammaEl";
  id: string;
  newGammaEl: number;
}

export const parseChangeAssessmentGammaEl: NormWireReader<ChangeAssessmentGammaEl> = normWireObject<ChangeAssessmentGammaEl>({ mutation: normWireRequired(normWireLiteral("changeAssessmentGammaEl")), id: normWireRequired(normWireString), newGammaEl: normWireRequired(normWireNumber) });
