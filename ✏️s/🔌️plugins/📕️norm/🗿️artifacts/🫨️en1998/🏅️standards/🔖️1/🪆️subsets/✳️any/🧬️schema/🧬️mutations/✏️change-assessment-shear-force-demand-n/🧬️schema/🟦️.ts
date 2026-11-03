/** ✏️ `change-assessment-shear-force-demand-n` wire twin: the leaf payload `ChangeAssessmentShearForceDemandN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentShearForceDemandN {
  mutation: "changeAssessmentShearForceDemandN";
  id: string;
  newShearForceDemandN: number;
}

export const parseChangeAssessmentShearForceDemandN: NormWireReader<ChangeAssessmentShearForceDemandN> = normWireObject<ChangeAssessmentShearForceDemandN>({ mutation: normWireRequired(normWireLiteral("changeAssessmentShearForceDemandN")), id: normWireRequired(normWireString), newShearForceDemandN: normWireRequired(normWireNumber) });
