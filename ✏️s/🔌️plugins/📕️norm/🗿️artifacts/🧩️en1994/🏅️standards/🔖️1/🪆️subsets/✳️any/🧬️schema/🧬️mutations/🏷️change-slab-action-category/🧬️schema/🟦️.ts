/** 🏷️ `change-slab-action-category` wire twin: the leaf payload `ChangeSlabActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabActionCategory {
  slabId: string;
  id: string;
  newCategory: string;
}

export const parseChangeSlabActionCategory: NormWireReader<ChangeSlabActionCategory> = normWireObject<ChangeSlabActionCategory>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });
