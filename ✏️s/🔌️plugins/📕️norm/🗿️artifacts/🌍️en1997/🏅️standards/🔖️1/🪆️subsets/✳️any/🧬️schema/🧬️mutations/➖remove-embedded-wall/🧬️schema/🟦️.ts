/** ➖ `remove-embedded-wall` wire twin: the leaf payload `RemoveEmbeddedWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveEmbeddedWall {
  mutation: "removeEmbeddedWall";
  index: number;
}

export const parseRemoveEmbeddedWall: NormWireReader<RemoveEmbeddedWall> = normWireObject<RemoveEmbeddedWall>({ mutation: normWireRequired(normWireLiteral("removeEmbeddedWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
