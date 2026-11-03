/** 🔷️ `change-vertex-y` wire twin: the leaf payload `ChangeVertexY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVertexY {
  mutation: "changeVertexY";
  outlineVertexId: string;
  newY: number;
}

export const parseChangeVertexY: NormWireReader<ChangeVertexY> = normWireObject<ChangeVertexY>({ mutation: normWireRequired(normWireLiteral("changeVertexY")), outlineVertexId: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
