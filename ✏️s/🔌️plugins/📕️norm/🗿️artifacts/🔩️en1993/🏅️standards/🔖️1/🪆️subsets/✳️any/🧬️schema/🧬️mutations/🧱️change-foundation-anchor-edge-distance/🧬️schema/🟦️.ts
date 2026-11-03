/** 🧱️ `change-foundation-anchor-edge-distance` wire twin: the leaf payload `ChangeFoundationAnchorEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorEdgeDistance {
  id: string;
  newAnchorEdgeDistance: number;
}

export const parseChangeFoundationAnchorEdgeDistance: NormWireReader<ChangeFoundationAnchorEdgeDistance> = normWireObject<ChangeFoundationAnchorEdgeDistance>({ id: normWireRequired(normWireString), newAnchorEdgeDistance: normWireRequired(normWireNumber) });
