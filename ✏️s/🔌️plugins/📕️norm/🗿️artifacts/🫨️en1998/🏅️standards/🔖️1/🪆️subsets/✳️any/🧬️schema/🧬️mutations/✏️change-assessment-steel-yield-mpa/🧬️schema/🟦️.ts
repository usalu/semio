/** ✏️ `change-assessment-steel-yield-mpa` wire twin: the leaf payload `ChangeAssessmentSteelYieldMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentSteelYieldMpa {
  mutation: "changeAssessmentSteelYieldMpa";
  id: string;
  newSteelYieldMpa: number;
}

export const parseChangeAssessmentSteelYieldMpa: NormWireReader<ChangeAssessmentSteelYieldMpa> = normWireObject<ChangeAssessmentSteelYieldMpa>({ mutation: normWireRequired(normWireLiteral("changeAssessmentSteelYieldMpa")), id: normWireRequired(normWireString), newSteelYieldMpa: normWireRequired(normWireNumber) });
