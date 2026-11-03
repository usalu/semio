/** ✏️ `change-assessment-bar-diameter-mm` wire twin: the leaf payload `ChangeAssessmentBarDiameterMm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentBarDiameterMm {
  mutation: "changeAssessmentBarDiameterMm";
  id: string;
  newBarDiameterMm: number;
}

export const parseChangeAssessmentBarDiameterMm: NormWireReader<ChangeAssessmentBarDiameterMm> = normWireObject<ChangeAssessmentBarDiameterMm>({ mutation: normWireRequired(normWireLiteral("changeAssessmentBarDiameterMm")), id: normWireRequired(normWireString), newBarDiameterMm: normWireRequired(normWireNumber) });
