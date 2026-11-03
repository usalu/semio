/** 🪚️ `change-chase-x` wire twin: the leaf payload `ChangeChaseX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseX {
  id: string;
  newXM: number;
}

export const parseChangeChaseX: NormWireReader<ChangeChaseX> = normWireObject<ChangeChaseX>({ id: normWireRequired(normWireString), newXM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
