/** 🪜️ `change-built-up-chord-width` wire twin: the leaf payload `ChangeBuiltUpChordWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordWidth {
  id: string;
  newChordWidth: number;
}

export const parseChangeBuiltUpChordWidth: NormWireReader<ChangeBuiltUpChordWidth> = normWireObject<ChangeBuiltUpChordWidth>({ id: normWireRequired(normWireString), newChordWidth: normWireRequired(normWireNumber) });
