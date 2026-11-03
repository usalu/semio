/** ⚓️ `change-anchor-wall-id` wire twin: the leaf payload `ChangeAnchorWallId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorWallId {
  mutation: "changeAnchorWallId";
  id: string;
  newWallId: string;
}

export const parseChangeAnchorWallId: NormWireReader<ChangeAnchorWallId> = normWireObject<ChangeAnchorWallId>({ mutation: normWireRequired(normWireLiteral("changeAnchorWallId")), id: normWireRequired(normWireString), newWallId: normWireRequired(normWireString) });
