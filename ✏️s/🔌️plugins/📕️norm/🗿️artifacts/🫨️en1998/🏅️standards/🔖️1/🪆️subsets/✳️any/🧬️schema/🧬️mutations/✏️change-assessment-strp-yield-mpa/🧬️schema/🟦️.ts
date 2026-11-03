/** ✏️ `change-assessment-strp-yield-mpa` wire twin: the leaf payload `ChangeAssessmentStrpYieldMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentStrpYieldMpa {
  mutation: "changeAssessmentStrpYieldMpa";
  id: string;
  newStirrupYieldMpa: number;
}

export const parseChangeAssessmentStrpYieldMpa: NormWireReader<ChangeAssessmentStrpYieldMpa> = normWireObject<ChangeAssessmentStrpYieldMpa>({ mutation: normWireRequired(normWireLiteral("changeAssessmentStrpYieldMpa")), id: normWireRequired(normWireString), newStirrupYieldMpa: normWireRequired(normWireNumber) });
