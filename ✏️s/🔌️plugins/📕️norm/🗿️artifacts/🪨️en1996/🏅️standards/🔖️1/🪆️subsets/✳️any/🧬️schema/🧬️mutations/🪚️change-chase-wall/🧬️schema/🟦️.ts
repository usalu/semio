/** 🪚️ `change-chase-wall` wire twin: the leaf payload `ChangeChaseWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseWall {
  id: string;
  newWallId: string;
}

export const parseChangeChaseWall: NormWireReader<ChangeChaseWall> = normWireObject<ChangeChaseWall>({ id: normWireRequired(normWireString), newWallId: normWireRequired(normWireString) });
