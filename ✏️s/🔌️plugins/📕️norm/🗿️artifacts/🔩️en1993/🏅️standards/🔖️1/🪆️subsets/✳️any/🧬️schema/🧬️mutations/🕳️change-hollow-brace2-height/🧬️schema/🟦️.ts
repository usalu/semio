/** 🕳️ `change-hollow-brace2-height` wire twin: the leaf payload `ChangeHollowBrace2Height`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace2Height {
  id: string;
  newBrace2Height: number;
}

export const parseChangeHollowBrace2Height: NormWireReader<ChangeHollowBrace2Height> = normWireObject<ChangeHollowBrace2Height>({ id: normWireRequired(normWireString), newBrace2Height: normWireRequired(normWireNumber) });
