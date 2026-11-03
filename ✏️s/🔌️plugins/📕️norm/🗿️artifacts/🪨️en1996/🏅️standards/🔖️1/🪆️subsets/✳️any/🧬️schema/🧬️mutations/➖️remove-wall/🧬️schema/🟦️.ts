/** ➖️ `remove-wall` wire twin: the leaf payload `RemoveWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveWall {
  id: string;
}

export const parseRemoveWall: NormWireReader<RemoveWall> = normWireObject<RemoveWall>({ id: normWireRequired(normWireString) });
