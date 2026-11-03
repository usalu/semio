/** 🧮️ `change-slab-k-factor` wire twin: the leaf payload `ChangeSlabKFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabKFactor {
  id: string;
  newKFactor: number;
}

export const parseChangeSlabKFactor: NormWireReader<ChangeSlabKFactor> = normWireObject<ChangeSlabKFactor>({ id: normWireRequired(normWireString), newKFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
