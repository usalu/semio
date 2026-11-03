/** 🧱️ `change-foundation-anchor-class` wire twin: the leaf payload `ChangeFoundationAnchorClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorClass {
  id: string;
  newAnchorClass: string;
}

export const parseChangeFoundationAnchorClass: NormWireReader<ChangeFoundationAnchorClass> = normWireObject<ChangeFoundationAnchorClass>({ id: normWireRequired(normWireString), newAnchorClass: normWireRequired(normWireString) });
