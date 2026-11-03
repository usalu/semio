/** 🕳️ `change-hollow-chord-thickness` wire twin: the leaf payload `ChangeHollowChordThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowChordThickness {
  id: string;
  newChordThickness: number;
}

export const parseChangeHollowChordThickness: NormWireReader<ChangeHollowChordThickness> = normWireObject<ChangeHollowChordThickness>({ id: normWireRequired(normWireString), newChordThickness: normWireRequired(normWireNumber) });
