/** ➖️ `remove-building` wire twin: the leaf payload `RemoveBuilding`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBuilding {
  mutation: "removeBuilding";
  id: string;
}

export const parseRemoveBuilding: NormWireReader<RemoveBuilding> = normWireObject<RemoveBuilding>({ mutation: normWireRequired(normWireLiteral("removeBuilding")), id: normWireRequired(normWireString) });
