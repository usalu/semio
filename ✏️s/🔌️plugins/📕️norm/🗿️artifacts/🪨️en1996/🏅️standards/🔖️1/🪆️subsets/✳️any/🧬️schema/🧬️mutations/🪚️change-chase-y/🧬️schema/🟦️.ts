/** 🪚️ `change-chase-y` wire twin: the leaf payload `ChangeChaseY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseY {
  id: string;
  newYM: number;
}

export const parseChangeChaseY: NormWireReader<ChangeChaseY> = normWireObject<ChangeChaseY>({ id: normWireRequired(normWireString), newYM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
