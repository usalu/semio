/** 🪜️ `change-built-up-chord-distance` wire twin: the leaf payload `ChangeBuiltUpChordDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordDistance {
  id: string;
  newChordDistance: number;
}

export const parseChangeBuiltUpChordDistance: NormWireReader<ChangeBuiltUpChordDistance> = normWireObject<ChangeBuiltUpChordDistance>({ id: normWireRequired(normWireString), newChordDistance: normWireRequired(normWireNumber) });
