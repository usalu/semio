/** 🔷️ `change-vertex-z` wire twin: the leaf payload `ChangeVertexZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVertexZ {
  mutation: "changeVertexZ";
  outlineVertexId: string;
  newZ: number;
}

export const parseChangeVertexZ: NormWireReader<ChangeVertexZ> = normWireObject<ChangeVertexZ>({ mutation: normWireRequired(normWireLiteral("changeVertexZ")), outlineVertexId: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
