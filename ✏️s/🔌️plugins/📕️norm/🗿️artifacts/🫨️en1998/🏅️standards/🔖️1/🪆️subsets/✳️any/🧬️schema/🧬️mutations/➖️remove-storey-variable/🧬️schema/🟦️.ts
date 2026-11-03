/** ➖️ `remove-storey-variable` wire twin: the leaf payload `RemoveStoreyVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveStoreyVariable {
  mutation: "removeStoreyVariable";
  buildingId: string;
  storeyId: string;
  id: string;
}

export const parseRemoveStoreyVariable: NormWireReader<RemoveStoreyVariable> = normWireObject<RemoveStoreyVariable>({ mutation: normWireRequired(normWireLiteral("removeStoreyVariable")), buildingId: normWireRequired(normWireString), storeyId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
