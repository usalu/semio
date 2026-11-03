/** ⚙ `remove-rail-load-group` wire twin: the leaf payload `RemoveRailLoadGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRailLoadGroup {
  id: string;
}

export const parseRemoveRailLoadGroup: NormWireReader<RemoveRailLoadGroup> = normWireObject<RemoveRailLoadGroup>({ id: normWireRequired(normWireString) });
