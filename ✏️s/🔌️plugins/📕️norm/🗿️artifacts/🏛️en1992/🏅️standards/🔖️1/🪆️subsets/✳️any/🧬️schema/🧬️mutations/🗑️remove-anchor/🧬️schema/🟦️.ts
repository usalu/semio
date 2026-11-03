/** 🗑️ `remove-anchor` wire twin: the leaf payload `RemoveAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAnchor {
  anchorId: string;
}

export const parseRemoveAnchor: NormWireReader<RemoveAnchor> = normWireObject<RemoveAnchor>({ anchorId: normWireRequired(normWireString) });
