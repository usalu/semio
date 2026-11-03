/** 🗂️ `remove-outline-vertex` wire twin: the leaf payload `RemoveOutlineVertex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveOutlineVertex {
  mutation: "removeOutlineVertex";
  id: string;
}

export const parseRemoveOutlineVertex: NormWireReader<RemoveOutlineVertex> = normWireObject<RemoveOutlineVertex>({ mutation: normWireRequired(normWireLiteral("removeOutlineVertex")), id: normWireRequired(normWireString) });
