/** ✏️ `change-member-wall-thickness-m` wire twin: the leaf payload `ChangeMemberWallThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallThicknessM {
  mutation: "changeMemberWallThicknessM";
  buildingId: string;
  id: string;
  newWallThicknessM: number;
}

export const parseChangeMemberWallThicknessM: NormWireReader<ChangeMemberWallThicknessM> = normWireObject<ChangeMemberWallThicknessM>({ mutation: normWireRequired(normWireLiteral("changeMemberWallThicknessM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallThicknessM: normWireRequired(normWireNumber) });
