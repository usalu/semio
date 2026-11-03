/** ➖️ `remove-storey` wire twin: the leaf payload `RemoveStorey`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveStorey {
  mutation: "removeStorey";
  buildingId: string;
  id: string;
}

export const parseRemoveStorey: NormWireReader<RemoveStorey> = normWireObject<RemoveStorey>({ mutation: normWireRequired(normWireLiteral("removeStorey")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
