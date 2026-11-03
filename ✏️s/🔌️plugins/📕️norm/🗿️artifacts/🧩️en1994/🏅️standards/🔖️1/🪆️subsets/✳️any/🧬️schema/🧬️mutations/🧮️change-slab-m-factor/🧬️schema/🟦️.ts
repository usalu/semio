/** 🧮️ `change-slab-m-factor` wire twin: the leaf payload `ChangeSlabMFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabMFactor {
  id: string;
  newMFactor: number;
}

export const parseChangeSlabMFactor: NormWireReader<ChangeSlabMFactor> = normWireObject<ChangeSlabMFactor>({ id: normWireRequired(normWireString), newMFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
