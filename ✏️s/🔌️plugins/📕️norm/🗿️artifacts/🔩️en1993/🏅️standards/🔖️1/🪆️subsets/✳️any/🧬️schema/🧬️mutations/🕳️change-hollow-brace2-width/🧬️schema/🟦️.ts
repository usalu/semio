/** 🕳️ `change-hollow-brace2-width` wire twin: the leaf payload `ChangeHollowBrace2Width`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace2Width {
  id: string;
  newBrace2Width: number;
}

export const parseChangeHollowBrace2Width: NormWireReader<ChangeHollowBrace2Width> = normWireObject<ChangeHollowBrace2Width>({ id: normWireRequired(normWireString), newBrace2Width: normWireRequired(normWireNumber) });
