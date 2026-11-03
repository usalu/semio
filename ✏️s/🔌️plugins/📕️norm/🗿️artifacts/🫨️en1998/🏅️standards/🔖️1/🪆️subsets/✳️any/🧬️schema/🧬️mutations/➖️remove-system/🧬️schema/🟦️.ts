/** ➖️ `remove-system` wire twin: the leaf payload `RemoveSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSystem {
  mutation: "removeSystem";
  buildingId: string;
  id: string;
}

export const parseRemoveSystem: NormWireReader<RemoveSystem> = normWireObject<RemoveSystem>({ mutation: normWireRequired(normWireLiteral("removeSystem")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
