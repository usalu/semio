/** 🔷️ `change-vertex-section-id` wire twin: the leaf payload `ChangeVertexSectionId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVertexSectionId {
  mutation: "changeVertexSectionId";
  outlineVertexId: string;
  newSectionId: string;
}

export const parseChangeVertexSectionId: NormWireReader<ChangeVertexSectionId> = normWireObject<ChangeVertexSectionId>({ mutation: normWireRequired(normWireLiteral("changeVertexSectionId")), outlineVertexId: normWireRequired(normWireString), newSectionId: normWireRequired(normWireString) });
