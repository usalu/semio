/** ✏️ `change-building-storey-stiffness-ratio` wire twin: the leaf payload `ChangeBuildingStoreyStiffnessRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingStoreyStiffnessRatio {
  mutation: "changeBuildingStoreyStiffnessRatio";
  id: string;
  newStoreyStiffnessRatio: number;
}

export const parseChangeBuildingStoreyStiffnessRatio: NormWireReader<ChangeBuildingStoreyStiffnessRatio> = normWireObject<ChangeBuildingStoreyStiffnessRatio>({ mutation: normWireRequired(normWireLiteral("changeBuildingStoreyStiffnessRatio")), id: normWireRequired(normWireString), newStoreyStiffnessRatio: normWireRequired(normWireNumber) });
