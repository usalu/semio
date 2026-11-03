/** 🕳️ `change-hollow-brace1-angle` wire twin: the leaf payload `ChangeHollowBrace1Angle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowBrace1Angle {
  id: string;
  newBrace1Angle: number;
}

export const parseChangeHollowBrace1Angle: NormWireReader<ChangeHollowBrace1Angle> = normWireObject<ChangeHollowBrace1Angle>({ id: normWireRequired(normWireString), newBrace1Angle: normWireRequired(normWireNumber) });
