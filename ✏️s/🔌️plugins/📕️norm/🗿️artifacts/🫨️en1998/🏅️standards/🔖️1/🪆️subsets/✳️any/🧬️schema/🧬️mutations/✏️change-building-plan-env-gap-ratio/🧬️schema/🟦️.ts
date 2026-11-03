/** ✏️ `change-building-plan-env-gap-ratio` wire twin: the leaf payload `ChangeBuildingPlanEnvGapRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingPlanEnvGapRatio {
  mutation: "changeBuildingPlanEnvGapRatio";
  id: string;
  newPlanEnvelopeGapRatio: number;
}

export const parseChangeBuildingPlanEnvGapRatio: NormWireReader<ChangeBuildingPlanEnvGapRatio> = normWireObject<ChangeBuildingPlanEnvGapRatio>({ mutation: normWireRequired(normWireLiteral("changeBuildingPlanEnvGapRatio")), id: normWireRequired(normWireString), newPlanEnvelopeGapRatio: normWireRequired(normWireNumber) });
