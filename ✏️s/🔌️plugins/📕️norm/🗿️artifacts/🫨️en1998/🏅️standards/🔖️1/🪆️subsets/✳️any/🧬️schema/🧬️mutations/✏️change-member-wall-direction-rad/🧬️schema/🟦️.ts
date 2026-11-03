/** ✏️ `change-member-wall-direction-rad` wire twin: the leaf payload `ChangeMemberWallDirectionRad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallDirectionRad {
  mutation: "changeMemberWallDirectionRad";
  buildingId: string;
  id: string;
  newWallDirectionRad: number;
}

export const parseChangeMemberWallDirectionRad: NormWireReader<ChangeMemberWallDirectionRad> = normWireObject<ChangeMemberWallDirectionRad>({ mutation: normWireRequired(normWireLiteral("changeMemberWallDirectionRad")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallDirectionRad: normWireRequired(normWireNumber) });
