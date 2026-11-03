/** 🪜️ `change-built-up-chord-flange-thickness` wire twin: the leaf payload `ChangeBuiltUpChordFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordFlangeThickness {
  id: string;
  newChordFlangeThickness: number;
}

export const parseChangeBuiltUpChordFlangeThickness: NormWireReader<ChangeBuiltUpChordFlangeThickness> = normWireObject<ChangeBuiltUpChordFlangeThickness>({ id: normWireRequired(normWireString), newChordFlangeThickness: normWireRequired(normWireNumber) });
