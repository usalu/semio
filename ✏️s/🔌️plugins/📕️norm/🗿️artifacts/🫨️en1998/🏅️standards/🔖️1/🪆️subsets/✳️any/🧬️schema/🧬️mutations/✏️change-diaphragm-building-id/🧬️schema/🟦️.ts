/** ✏️ `change-diaphragm-building-id` wire twin: the leaf payload `ChangeDiaphragmBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmBuildingId {
  mutation: "changeDiaphragmBuildingId";
  id: string;
  newBuildingId: string;
}

export const parseChangeDiaphragmBuildingId: NormWireReader<ChangeDiaphragmBuildingId> = normWireObject<ChangeDiaphragmBuildingId>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmBuildingId")), id: normWireRequired(normWireString), newBuildingId: normWireRequired(normWireString) });
