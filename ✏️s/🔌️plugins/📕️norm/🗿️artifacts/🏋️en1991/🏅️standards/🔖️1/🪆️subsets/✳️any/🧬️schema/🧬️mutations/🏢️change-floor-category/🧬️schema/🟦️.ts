/** 🏢 `change-floor-category` wire twin: the leaf payload `ChangeFloorCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorCategory {
  id: string;
  newCategory: string;
}

export const parseChangeFloorCategory: NormWireReader<ChangeFloorCategory> = normWireObject<ChangeFloorCategory>({ id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });
