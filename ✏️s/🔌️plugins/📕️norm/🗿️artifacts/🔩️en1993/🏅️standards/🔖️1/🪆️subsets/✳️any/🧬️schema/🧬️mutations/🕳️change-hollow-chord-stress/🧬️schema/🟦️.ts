/** 🕳️ `change-hollow-chord-stress` wire twin: the leaf payload `ChangeHollowChordStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowChordStress {
  id: string;
  newChordStress: number;
}

export const parseChangeHollowChordStress: NormWireReader<ChangeHollowChordStress> = normWireObject<ChangeHollowChordStress>({ id: normWireRequired(normWireString), newChordStress: normWireRequired(normWireNumber) });
