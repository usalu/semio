/** ✏️ `change-member-wall-start-ym` wire twin: the leaf payload `ChangeMemberWallStartYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallStartYM {
  mutation: "changeMemberWallStartYM";
  buildingId: string;
  id: string;
  newWallStartYM: number;
}

export const parseChangeMemberWallStartYM: NormWireReader<ChangeMemberWallStartYM> = normWireObject<ChangeMemberWallStartYM>({ mutation: normWireRequired(normWireLiteral("changeMemberWallStartYM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallStartYM: normWireRequired(normWireNumber) });
