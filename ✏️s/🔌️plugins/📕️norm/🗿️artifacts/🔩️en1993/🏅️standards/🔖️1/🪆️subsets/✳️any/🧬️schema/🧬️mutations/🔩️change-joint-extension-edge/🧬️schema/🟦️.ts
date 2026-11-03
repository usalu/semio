/** 🔩️ `change-joint-extension-edge` wire twin: the leaf payload `ChangeJointExtensionEdge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointExtensionEdge {
  id: string;
  newExtensionEdge: number;
}

export const parseChangeJointExtensionEdge: NormWireReader<ChangeJointExtensionEdge> = normWireObject<ChangeJointExtensionEdge>({ id: normWireRequired(normWireString), newExtensionEdge: normWireRequired(normWireNumber) });
