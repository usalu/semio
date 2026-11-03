/** 🔷️ `change-vertex-ring` wire twin: the leaf payload `ChangeVertexRing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type OutlineRing, parseOutlineRing } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeVertexRing {
  mutation: "changeVertexRing";
  outlineVertexId: string;
  newRing: OutlineRing;
}

export const parseChangeVertexRing: NormWireReader<ChangeVertexRing> = normWireObject<ChangeVertexRing>({ mutation: normWireRequired(normWireLiteral("changeVertexRing")), outlineVertexId: normWireRequired(normWireString), newRing: normWireRequired(parseOutlineRing) });
