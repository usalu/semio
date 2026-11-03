/** 🕳️ `change-hollow-brace1-height` wire twin: the leaf payload `ChangeHollowBrace1Height`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace1Height {
  id: string;
  newBrace1Height: number;
}

export const parseChangeHollowBrace1Height: NormWireReader<ChangeHollowBrace1Height> = normWireObject<ChangeHollowBrace1Height>({ id: normWireRequired(normWireString), newBrace1Height: normWireRequired(normWireNumber) });
