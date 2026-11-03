/** 🕸️ `change-frame-node-load-x` wire twin: the leaf payload `ChangeFrameNodeLoadX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameNodeLoadX {
  id: string;
  newLoadX: number;
}

export const parseChangeFrameNodeLoadX: NormWireReader<ChangeFrameNodeLoadX> = normWireObject<ChangeFrameNodeLoadX>({ id: normWireRequired(normWireString), newLoadX: normWireRequired(normWireNumber) });
