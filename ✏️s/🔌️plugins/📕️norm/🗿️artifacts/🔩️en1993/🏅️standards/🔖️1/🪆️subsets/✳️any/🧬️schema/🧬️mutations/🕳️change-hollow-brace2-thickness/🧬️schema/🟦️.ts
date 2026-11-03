/** 🕳️ `change-hollow-brace2-thickness` wire twin: the leaf payload `ChangeHollowBrace2Thickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace2Thickness {
  id: string;
  newBrace2Thickness: number;
}

export const parseChangeHollowBrace2Thickness: NormWireReader<ChangeHollowBrace2Thickness> = normWireObject<ChangeHollowBrace2Thickness>({ id: normWireRequired(normWireString), newBrace2Thickness: normWireRequired(normWireNumber) });
