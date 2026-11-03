/** ✏️ `change-building-plan-length-m` wire twin: the leaf payload `ChangeBuildingPlanLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingPlanLengthM {
  mutation: "changeBuildingPlanLengthM";
  id: string;
  newPlanLengthM: number;
}

export const parseChangeBuildingPlanLengthM: NormWireReader<ChangeBuildingPlanLengthM> = normWireObject<ChangeBuildingPlanLengthM>({ mutation: normWireRequired(normWireLiteral("changeBuildingPlanLengthM")), id: normWireRequired(normWireString), newPlanLengthM: normWireRequired(normWireNumber) });
