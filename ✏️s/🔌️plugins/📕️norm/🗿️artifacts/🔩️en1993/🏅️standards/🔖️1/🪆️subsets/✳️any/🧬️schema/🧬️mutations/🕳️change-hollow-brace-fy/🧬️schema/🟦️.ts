/** 🕳️ `change-hollow-brace-fy` wire twin: the leaf payload `ChangeHollowBraceFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBraceFy {
  id: string;
  newBraceFy: number;
}

export const parseChangeHollowBraceFy: NormWireReader<ChangeHollowBraceFy> = normWireObject<ChangeHollowBraceFy>({ id: normWireRequired(normWireString), newBraceFy: normWireRequired(normWireNumber) });
