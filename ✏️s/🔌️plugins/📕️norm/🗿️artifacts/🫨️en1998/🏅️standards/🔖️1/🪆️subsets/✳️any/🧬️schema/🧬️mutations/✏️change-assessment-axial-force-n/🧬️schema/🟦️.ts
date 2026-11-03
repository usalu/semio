/** ✏️ `change-assessment-axial-force-n` wire twin: the leaf payload `ChangeAssessmentAxialForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentAxialForceN {
  mutation: "changeAssessmentAxialForceN";
  id: string;
  newAxialForceN: number;
}

export const parseChangeAssessmentAxialForceN: NormWireReader<ChangeAssessmentAxialForceN> = normWireObject<ChangeAssessmentAxialForceN>({ mutation: normWireRequired(normWireLiteral("changeAssessmentAxialForceN")), id: normWireRequired(normWireString), newAxialForceN: normWireRequired(normWireNumber) });
