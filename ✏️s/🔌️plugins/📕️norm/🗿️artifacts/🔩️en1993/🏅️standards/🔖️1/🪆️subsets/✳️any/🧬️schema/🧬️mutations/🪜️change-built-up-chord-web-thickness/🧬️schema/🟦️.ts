/** 🪜️ `change-built-up-chord-web-thickness` wire twin: the leaf payload `ChangeBuiltUpChordWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordWebThickness {
  id: string;
  newChordWebThickness: number;
}

export const parseChangeBuiltUpChordWebThickness: NormWireReader<ChangeBuiltUpChordWebThickness> = normWireObject<ChangeBuiltUpChordWebThickness>({ id: normWireRequired(normWireString), newChordWebThickness: normWireRequired(normWireNumber) });
