/** 🪨️ `change-slab-f-ck-pa` wire twin: the leaf payload `ChangeSlabFCkPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabFCkPa {
  id: string;
  newFCkPa: number;
}

export const parseChangeSlabFCkPa: NormWireReader<ChangeSlabFCkPa> = normWireObject<ChangeSlabFCkPa>({ id: normWireRequired(normWireString), newFCkPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
