/** ➖ `remove-retaining-wall` wire twin: the leaf payload `RemoveRetainingWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRetainingWall {
  mutation: "removeRetainingWall";
  index: number;
}

export const parseRemoveRetainingWall: NormWireReader<RemoveRetainingWall> = normWireObject<RemoveRetainingWall>({ mutation: normWireRequired(normWireLiteral("removeRetainingWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
