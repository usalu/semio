/** 🕳️ `change-hollow-brace1-force` wire twin: the leaf payload `ChangeHollowBrace1Force`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace1Force {
  id: string;
  newBrace1Force: number;
}

export const parseChangeHollowBrace1Force: NormWireReader<ChangeHollowBrace1Force> = normWireObject<ChangeHollowBrace1Force>({ id: normWireRequired(normWireString), newBrace1Force: normWireRequired(normWireNumber) });
