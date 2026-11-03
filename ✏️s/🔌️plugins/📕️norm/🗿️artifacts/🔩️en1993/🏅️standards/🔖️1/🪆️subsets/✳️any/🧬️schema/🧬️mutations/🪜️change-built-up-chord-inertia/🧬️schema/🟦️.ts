/** 🪜️ `change-built-up-chord-inertia` wire twin: the leaf payload `ChangeBuiltUpChordInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChordInertia {
  id: string;
  newChordInertia: number;
}

export const parseChangeBuiltUpChordInertia: NormWireReader<ChangeBuiltUpChordInertia> = normWireObject<ChangeBuiltUpChordInertia>({ id: normWireRequired(normWireString), newChordInertia: normWireRequired(normWireNumber) });
