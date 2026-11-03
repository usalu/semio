/** 🧱️ `change-foundation-anchor-embedment` wire twin: the leaf payload `ChangeFoundationAnchorEmbedment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorEmbedment {
  id: string;
  newAnchorEmbedment: number;
}

export const parseChangeFoundationAnchorEmbedment: NormWireReader<ChangeFoundationAnchorEmbedment> = normWireObject<ChangeFoundationAnchorEmbedment>({ id: normWireRequired(normWireString), newAnchorEmbedment: normWireRequired(normWireNumber) });
