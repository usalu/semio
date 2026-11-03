/** 🪜️ `change-built-up-chord-flange-thk` wire twin: the leaf payload `ChangeBuiltUpChordFlangeThk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordFlangeThk {
  id: string;
  newChordFlangeThickness: number;
}

export const parseChangeBuiltUpChordFlangeThk: NormWireReader<ChangeBuiltUpChordFlangeThk> = normWireObject<ChangeBuiltUpChordFlangeThk>({ id: normWireRequired(normWireString), newChordFlangeThickness: normWireRequired(normWireNumber) });
