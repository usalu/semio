/** 🛤 `remove-rail-dynamic` wire twin: the leaf payload `RemoveRailDynamic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRailDynamic {
  id: string;
}

export const parseRemoveRailDynamic: NormWireReader<RemoveRailDynamic> = normWireObject<RemoveRailDynamic>({ id: normWireRequired(normWireString) });
