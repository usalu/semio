/** ➖️ `remove-weld-group` wire twin: the leaf payload `RemoveWeldGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveWeldGroup {
  index: number;
}

export const parseRemoveWeldGroup: NormWireReader<RemoveWeldGroup> = normWireObject<RemoveWeldGroup>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
