/** ✏️ `change-assessment-chord-yield-rad` wire twin: the leaf payload `ChangeAssessmentChordYieldRad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentChordYieldRad {
  mutation: "changeAssessmentChordYieldRad";
  id: string;
  newChordRotationYieldRad: number;
}

export const parseChangeAssessmentChordYieldRad: NormWireReader<ChangeAssessmentChordYieldRad> = normWireObject<ChangeAssessmentChordYieldRad>({ mutation: normWireRequired(normWireLiteral("changeAssessmentChordYieldRad")), id: normWireRequired(normWireString), newChordRotationYieldRad: normWireRequired(normWireNumber) });
