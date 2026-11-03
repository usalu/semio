/** 🪜️ `change-built-up-chord-depth` wire twin: the leaf payload `ChangeBuiltUpChordDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordDepth {
  id: string;
  newChordDepth: number;
}

export const parseChangeBuiltUpChordDepth: NormWireReader<ChangeBuiltUpChordDepth> = normWireObject<ChangeBuiltUpChordDepth>({ id: normWireRequired(normWireString), newChordDepth: normWireRequired(normWireNumber) });
