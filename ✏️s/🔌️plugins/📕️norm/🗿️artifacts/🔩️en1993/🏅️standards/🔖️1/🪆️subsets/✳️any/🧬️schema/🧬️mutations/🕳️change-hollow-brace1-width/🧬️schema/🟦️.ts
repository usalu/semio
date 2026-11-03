/** 🕳️ `change-hollow-brace1-width` wire twin: the leaf payload `ChangeHollowBrace1Width`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace1Width {
  id: string;
  newBrace1Width: number;
}

export const parseChangeHollowBrace1Width: NormWireReader<ChangeHollowBrace1Width> = normWireObject<ChangeHollowBrace1Width>({ id: normWireRequired(normWireString), newBrace1Width: normWireRequired(normWireNumber) });
