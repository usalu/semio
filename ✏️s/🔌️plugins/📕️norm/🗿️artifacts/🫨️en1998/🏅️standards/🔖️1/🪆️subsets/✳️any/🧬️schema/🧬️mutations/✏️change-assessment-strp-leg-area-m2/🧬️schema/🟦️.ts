/** ✏️ `change-assessment-strp-leg-area-m2` wire twin: the leaf payload `ChangeAssessmentStrpLegAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentStrpLegAreaM2 {
  mutation: "changeAssessmentStrpLegAreaM2";
  id: string;
  newStirrupLegAreaM2: number;
}

export const parseChangeAssessmentStrpLegAreaM2: NormWireReader<ChangeAssessmentStrpLegAreaM2> = normWireObject<ChangeAssessmentStrpLegAreaM2>({ mutation: normWireRequired(normWireLiteral("changeAssessmentStrpLegAreaM2")), id: normWireRequired(normWireString), newStirrupLegAreaM2: normWireRequired(normWireNumber) });
