/** 🧱️ `change-foundation-anchor-stress-area` wire twin: the leaf payload `ChangeFoundationAnchorStressArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAnchorStressArea {
  id: string;
  newAnchorStressArea: number;
}

export const parseChangeFoundationAnchorStressArea: NormWireReader<ChangeFoundationAnchorStressArea> = normWireObject<ChangeFoundationAnchorStressArea>({ id: normWireRequired(normWireString), newAnchorStressArea: normWireRequired(normWireNumber) });
