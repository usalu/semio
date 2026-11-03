/** ✏️ `change-assessment-chord-ult-rad` wire twin: the leaf payload `ChangeAssessmentChordUltRad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentChordUltRad {
  mutation: "changeAssessmentChordUltRad";
  id: string;
  newChordRotationUltimateRad: number;
}

export const parseChangeAssessmentChordUltRad: NormWireReader<ChangeAssessmentChordUltRad> = normWireObject<ChangeAssessmentChordUltRad>({ mutation: normWireRequired(normWireLiteral("changeAssessmentChordUltRad")), id: normWireRequired(normWireString), newChordRotationUltimateRad: normWireRequired(normWireNumber) });
