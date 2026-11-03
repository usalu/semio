/** 🕳️ `change-hollow-chord-height` wire twin: the leaf payload `ChangeHollowChordHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowChordHeight {
  id: string;
  newChordHeight: number;
}

export const parseChangeHollowChordHeight: NormWireReader<ChangeHollowChordHeight> = normWireObject<ChangeHollowChordHeight>({ id: normWireRequired(normWireString), newChordHeight: normWireRequired(normWireNumber) });
