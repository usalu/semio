/** ✏️ `change-member-role` wire twin: the leaf payload `ChangeMemberRole`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRole {
  mutation: "changeMemberRole";
  buildingId: string;
  id: string;
  newRole: "beam" | "column" | "wall" | "brace";
}

export const parseChangeMemberRole: NormWireReader<ChangeMemberRole> = normWireObject<ChangeMemberRole>({ mutation: normWireRequired(normWireLiteral("changeMemberRole")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newRole: normWireRequired(normWireLiteral("beam", "column", "wall", "brace")) });
