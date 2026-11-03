/** 🕳️ `change-hollow-brace2-force` wire twin: the leaf payload `ChangeHollowBrace2Force`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace2Force {
  id: string;
  newBrace2Force: number;
}

export const parseChangeHollowBrace2Force: NormWireReader<ChangeHollowBrace2Force> = normWireObject<ChangeHollowBrace2Force>({ id: normWireRequired(normWireString), newBrace2Force: normWireRequired(normWireNumber) });
