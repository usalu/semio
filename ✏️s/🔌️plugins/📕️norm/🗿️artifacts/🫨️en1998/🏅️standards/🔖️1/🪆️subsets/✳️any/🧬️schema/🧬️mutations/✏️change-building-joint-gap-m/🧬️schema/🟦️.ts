/** ✏️ `change-building-joint-gap-m` wire twin: the leaf payload `ChangeBuildingJointGapM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingJointGapM {
  mutation: "changeBuildingJointGapM";
  id: string;
  newJointGapM: number;
}

export const parseChangeBuildingJointGapM: NormWireReader<ChangeBuildingJointGapM> = normWireObject<ChangeBuildingJointGapM>({ mutation: normWireRequired(normWireLiteral("changeBuildingJointGapM")), id: normWireRequired(normWireString), newJointGapM: normWireRequired(normWireNumber) });
