/** 🕳️ `change-hollow-brace1-thickness` wire twin: the leaf payload `ChangeHollowBrace1Thickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace1Thickness {
  id: string;
  newBrace1Thickness: number;
}

export const parseChangeHollowBrace1Thickness: NormWireReader<ChangeHollowBrace1Thickness> = normWireObject<ChangeHollowBrace1Thickness>({ id: normWireRequired(normWireString), newBrace1Thickness: normWireRequired(normWireNumber) });
