/** ✏️ `change-member-wall-length-m` wire twin: the leaf payload `ChangeMemberWallLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallLengthM {
  mutation: "changeMemberWallLengthM";
  buildingId: string;
  id: string;
  newWallLengthM: number;
}

export const parseChangeMemberWallLengthM: NormWireReader<ChangeMemberWallLengthM> = normWireObject<ChangeMemberWallLengthM>({ mutation: normWireRequired(normWireLiteral("changeMemberWallLengthM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallLengthM: normWireRequired(normWireNumber) });
