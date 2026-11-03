/** 🪜️ `change-built-up-chord-area` wire twin: the leaf payload `ChangeBuiltUpChordArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordArea {
  id: string;
  newChordArea: number;
}

export const parseChangeBuiltUpChordArea: NormWireReader<ChangeBuiltUpChordArea> = normWireObject<ChangeBuiltUpChordArea>({ id: normWireRequired(normWireString), newChordArea: normWireRequired(normWireNumber) });
