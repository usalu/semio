/** 🏋️ `change-gk-slab` wire twin: the leaf payload `ChangeGKSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGKSlab {
  wallId: string;
  id: string;
  newGKSlabN: number;
}

export const parseChangeGKSlab: NormWireReader<ChangeGKSlab> = normWireObject<ChangeGKSlab>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newGKSlabN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
