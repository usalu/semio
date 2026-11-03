/** ✏️ `change-assessment-fc-mpa` wire twin: the leaf payload `ChangeAssessmentFcMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentFcMpa {
  mutation: "changeAssessmentFcMpa";
  id: string;
  newConcreteStrengthMpa: number;
}

export const parseChangeAssessmentFcMpa: NormWireReader<ChangeAssessmentFcMpa> = normWireObject<ChangeAssessmentFcMpa>({ mutation: normWireRequired(normWireLiteral("changeAssessmentFcMpa")), id: normWireRequired(normWireString), newConcreteStrengthMpa: normWireRequired(normWireNumber) });
