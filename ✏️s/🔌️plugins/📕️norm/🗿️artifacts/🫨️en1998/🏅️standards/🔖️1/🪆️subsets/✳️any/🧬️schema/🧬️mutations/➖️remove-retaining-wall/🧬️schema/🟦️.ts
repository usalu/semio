/** ➖️ `remove-retaining-wall` wire twin: the leaf payload `RemoveRetainingWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRetainingWall {
  mutation: "removeRetainingWall";
  id: string;
}

export const parseRemoveRetainingWall: NormWireReader<RemoveRetainingWall> = normWireObject<RemoveRetainingWall>({ mutation: normWireRequired(normWireLiteral("removeRetainingWall")), id: normWireRequired(normWireString) });
