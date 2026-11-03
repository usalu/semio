/** ✏️ `change-assessment-supported-building-id` wire twin: the leaf payload `ChangeAssessmentSupportedBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssessmentSupportedBuildingId {
  mutation: "changeAssessmentSupportedBuildingId";
  id: string;
  newSupportedBuildingId: string;
}

export const parseChangeAssessmentSupportedBuildingId: NormWireReader<ChangeAssessmentSupportedBuildingId> = normWireObject<ChangeAssessmentSupportedBuildingId>({ mutation: normWireRequired(normWireLiteral("changeAssessmentSupportedBuildingId")), id: normWireRequired(normWireString), newSupportedBuildingId: normWireRequired(normWireString) });
