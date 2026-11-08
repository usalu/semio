/** ➖️ `remove-isolation-system` wire twin: the leaf payload `RemoveIsolationSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveIsolationSystem {
  mutation: "removeIsolationSystem";
  id: string;
}

export const parseRemoveIsolationSystem: NormWireReader<RemoveIsolationSystem> = normWireObject<RemoveIsolationSystem>({ mutation: normWireRequired(normWireLiteral("removeIsolationSystem")), id: normWireRequired(normWireString) });
