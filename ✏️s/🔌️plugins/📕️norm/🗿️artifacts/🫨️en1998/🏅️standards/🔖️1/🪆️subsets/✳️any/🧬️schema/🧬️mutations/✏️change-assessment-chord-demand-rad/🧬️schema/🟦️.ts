/** ✏️ `change-assessment-chord-demand-rad` wire twin: the leaf payload `ChangeAssessmentChordDemandRad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentChordDemandRad {
  mutation: "changeAssessmentChordDemandRad";
  id: string;
  newChordRotationDemandRad: number;
}

export const parseChangeAssessmentChordDemandRad: NormWireReader<ChangeAssessmentChordDemandRad> = normWireObject<ChangeAssessmentChordDemandRad>({ mutation: normWireRequired(normWireLiteral("changeAssessmentChordDemandRad")), id: normWireRequired(normWireString), newChordRotationDemandRad: normWireRequired(normWireNumber) });
