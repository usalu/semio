/** 📏 `change-load-dispersal-slab-thickness` wire twin: the leaf payload `ChangeLoadDispersalSlabThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadDispersalSlabThickness {
  id: string;
  newSlabThickness: number;
}

export const parseChangeLoadDispersalSlabThickness: NormWireReader<ChangeLoadDispersalSlabThickness> = normWireObject<ChangeLoadDispersalSlabThickness>({ id: normWireRequired(normWireString), newSlabThickness: normWireRequired(normWireNumber) });
