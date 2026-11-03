/** 🔩️ `change-row-extension-edge` wire twin: the leaf payload `ChangeRowExtensionEdge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowExtensionEdge {
  id: string;
  newExtensionEdge: number;
}

export const parseChangeRowExtensionEdge: NormWireReader<ChangeRowExtensionEdge> = normWireObject<ChangeRowExtensionEdge>({ id: normWireRequired(normWireString), newExtensionEdge: normWireRequired(normWireNumber) });
