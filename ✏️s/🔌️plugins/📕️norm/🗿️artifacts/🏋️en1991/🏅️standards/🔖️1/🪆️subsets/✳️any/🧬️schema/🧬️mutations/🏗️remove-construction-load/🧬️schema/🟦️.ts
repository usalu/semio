/** 🏗 `remove-construction-load` wire twin: the leaf payload `RemoveConstructionLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConstructionLoad {
  id: string;
}

export const parseRemoveConstructionLoad: NormWireReader<RemoveConstructionLoad> = normWireObject<RemoveConstructionLoad>({ id: normWireRequired(normWireString) });
