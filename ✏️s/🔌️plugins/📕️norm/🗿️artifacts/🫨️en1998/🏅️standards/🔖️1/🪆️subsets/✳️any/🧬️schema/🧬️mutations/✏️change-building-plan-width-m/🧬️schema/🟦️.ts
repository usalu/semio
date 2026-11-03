/** ✏️ `change-building-plan-width-m` wire twin: the leaf payload `ChangeBuildingPlanWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingPlanWidthM {
  mutation: "changeBuildingPlanWidthM";
  id: string;
  newPlanWidthM: number;
}

export const parseChangeBuildingPlanWidthM: NormWireReader<ChangeBuildingPlanWidthM> = normWireObject<ChangeBuildingPlanWidthM>({ mutation: normWireRequired(normWireLiteral("changeBuildingPlanWidthM")), id: normWireRequired(normWireString), newPlanWidthM: normWireRequired(normWireNumber) });
