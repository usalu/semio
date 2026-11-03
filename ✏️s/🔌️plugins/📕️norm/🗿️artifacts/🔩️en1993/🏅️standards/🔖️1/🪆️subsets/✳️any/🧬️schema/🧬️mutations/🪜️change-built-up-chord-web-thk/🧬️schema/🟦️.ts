/** 🪜️ `change-built-up-chord-web-thk` wire twin: the leaf payload `ChangeBuiltUpChordWebThk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordWebThk {
  id: string;
  newChordWebThickness: number;
}

export const parseChangeBuiltUpChordWebThk: NormWireReader<ChangeBuiltUpChordWebThk> = normWireObject<ChangeBuiltUpChordWebThk>({ id: normWireRequired(normWireString), newChordWebThickness: normWireRequired(normWireNumber) });
