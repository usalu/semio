/** 🕸️ `change-frame-node-z` wire twin: the leaf payload `ChangeFrameNodeZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameNodeZ {
  id: string;
  newZ: number;
}

export const parseChangeFrameNodeZ: NormWireReader<ChangeFrameNodeZ> = normWireObject<ChangeFrameNodeZ>({ id: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
