/** ✏️ `change-member-wall-storey-id` wire twin: the leaf payload `ChangeMemberWallStoreyId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallStoreyId {
  mutation: "changeMemberWallStoreyId";
  buildingId: string;
  id: string;
  newWallStoreyId: string;
}

export const parseChangeMemberWallStoreyId: NormWireReader<ChangeMemberWallStoreyId> = normWireObject<ChangeMemberWallStoreyId>({ mutation: normWireRequired(normWireLiteral("changeMemberWallStoreyId")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallStoreyId: normWireRequired(normWireString) });
