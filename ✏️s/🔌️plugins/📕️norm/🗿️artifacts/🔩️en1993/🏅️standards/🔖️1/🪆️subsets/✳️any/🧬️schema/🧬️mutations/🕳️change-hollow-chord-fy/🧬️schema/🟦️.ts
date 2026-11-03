/** 🕳️ `change-hollow-chord-fy` wire twin: the leaf payload `ChangeHollowChordFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowChordFy {
  id: string;
  newChordFy: number;
}

export const parseChangeHollowChordFy: NormWireReader<ChangeHollowChordFy> = normWireObject<ChangeHollowChordFy>({ id: normWireRequired(normWireString), newChordFy: normWireRequired(normWireNumber) });
