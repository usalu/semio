/** 🕸️ `change-frame-node-load-z` wire twin: the leaf payload `ChangeFrameNodeLoadZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameNodeLoadZ {
  id: string;
  newLoadZ: number;
}

export const parseChangeFrameNodeLoadZ: NormWireReader<ChangeFrameNodeLoadZ> = normWireObject<ChangeFrameNodeLoadZ>({ id: normWireRequired(normWireString), newLoadZ: normWireRequired(normWireNumber) });
