/** 🕸️ `change-frame-node-x` wire twin: the leaf payload `ChangeFrameNodeX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameNodeX {
  id: string;
  newX: number;
}

export const parseChangeFrameNodeX: NormWireReader<ChangeFrameNodeX> = normWireObject<ChangeFrameNodeX>({ id: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
