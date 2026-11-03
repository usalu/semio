/** ✏️ `change-member-wall-start-xm` wire twin: the leaf payload `ChangeMemberWallStartXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallStartXM {
  mutation: "changeMemberWallStartXM";
  buildingId: string;
  id: string;
  newWallStartXM: number;
}

export const parseChangeMemberWallStartXM: NormWireReader<ChangeMemberWallStartXM> = normWireObject<ChangeMemberWallStartXM>({ mutation: normWireRequired(normWireLiteral("changeMemberWallStartXM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallStartXM: normWireRequired(normWireNumber) });
