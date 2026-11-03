/** ✏️ `change-assessment-shear-crack` wire twin: the leaf payload `ChangeAssessmentShearCrack`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentShearCrack {
  mutation: "changeAssessmentShearCrack";
  id: string;
  newShearCrackingBeforeYield: number;
}

export const parseChangeAssessmentShearCrack: NormWireReader<ChangeAssessmentShearCrack> = normWireObject<ChangeAssessmentShearCrack>({ mutation: normWireRequired(normWireLiteral("changeAssessmentShearCrack")), id: normWireRequired(normWireString), newShearCrackingBeforeYield: normWireRequired(normWireNumber) });
