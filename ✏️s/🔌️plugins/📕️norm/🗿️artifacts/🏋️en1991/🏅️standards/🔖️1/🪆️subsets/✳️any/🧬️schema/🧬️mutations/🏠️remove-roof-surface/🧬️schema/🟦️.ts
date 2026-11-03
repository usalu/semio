/** 🏠 `remove-roof-surface` wire twin: the leaf payload `RemoveRoofSurface`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRoofSurface {
  id: string;
}

export const parseRemoveRoofSurface: NormWireReader<RemoveRoofSurface> = normWireObject<RemoveRoofSurface>({ id: normWireRequired(normWireString) });
