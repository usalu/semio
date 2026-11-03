/** 🪜️ `change-built-up-chord-shape` wire twin: the leaf payload `ChangeBuiltUpChordShape`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChordShape, parseChordShape } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBuiltUpChordShape {
  id: string;
  newChordShape: ChordShape;
}

export const parseChangeBuiltUpChordShape: NormWireReader<ChangeBuiltUpChordShape> = normWireObject<ChangeBuiltUpChordShape>({ id: normWireRequired(normWireString), newChordShape: normWireRequired(parseChordShape) });
