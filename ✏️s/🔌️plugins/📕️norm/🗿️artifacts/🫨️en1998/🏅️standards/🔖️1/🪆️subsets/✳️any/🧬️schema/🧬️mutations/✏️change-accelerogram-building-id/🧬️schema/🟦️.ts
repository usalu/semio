/** ✏️ `change-accelerogram-building-id` wire twin: the leaf payload `ChangeAccelerogramBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccelerogramBuildingId {
  mutation: "changeAccelerogramBuildingId";
  id: string;
  newBuildingId: string;
}

export const parseChangeAccelerogramBuildingId: NormWireReader<ChangeAccelerogramBuildingId> = normWireObject<ChangeAccelerogramBuildingId>({ mutation: normWireRequired(normWireLiteral("changeAccelerogramBuildingId")), id: normWireRequired(normWireString), newBuildingId: normWireRequired(normWireString) });
