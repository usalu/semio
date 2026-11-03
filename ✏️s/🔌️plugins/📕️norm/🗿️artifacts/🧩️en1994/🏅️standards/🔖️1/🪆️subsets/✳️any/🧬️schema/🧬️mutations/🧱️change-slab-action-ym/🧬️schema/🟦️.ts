/** 🧱️ `change-slab-action-ym` wire twin: the leaf payload `ChangeSlabActionYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabActionYM {
  slabId: string;
  id: string;
  newYM: number;
}

export const parseChangeSlabActionYM: NormWireReader<ChangeSlabActionYM> = normWireObject<ChangeSlabActionYM>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newYM: normWireRequired(normWireNumber) });
