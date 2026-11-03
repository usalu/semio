/** 🔷️ `change-vertex-element-id` wire twin: the leaf payload `ChangeVertexElementId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVertexElementId {
  mutation: "changeVertexElementId";
  outlineVertexId: string;
  newElementId: string;
}

export const parseChangeVertexElementId: NormWireReader<ChangeVertexElementId> = normWireObject<ChangeVertexElementId>({ mutation: normWireRequired(normWireLiteral("changeVertexElementId")), outlineVertexId: normWireRequired(normWireString), newElementId: normWireRequired(normWireString) });
