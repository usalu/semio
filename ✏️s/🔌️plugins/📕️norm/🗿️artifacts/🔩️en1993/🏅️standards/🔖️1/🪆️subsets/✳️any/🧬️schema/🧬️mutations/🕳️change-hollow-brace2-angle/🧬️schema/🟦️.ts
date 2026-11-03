/** 🕳️ `change-hollow-brace2-angle` wire twin: the leaf payload `ChangeHollowBrace2Angle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace2Angle {
  id: string;
  newBrace2Angle: number;
}

export const parseChangeHollowBrace2Angle: NormWireReader<ChangeHollowBrace2Angle> = normWireObject<ChangeHollowBrace2Angle>({ id: normWireRequired(normWireString), newBrace2Angle: normWireRequired(normWireNumber) });
