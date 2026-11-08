/** ✏️ `change-critical-region-building-id` wire twin: the leaf payload `ChangeCriticalRegionBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionBuildingId {
  mutation: "changeCriticalRegionBuildingId";
  id: string;
  newBuildingId: string;
}

export const parseChangeCriticalRegionBuildingId: NormWireReader<ChangeCriticalRegionBuildingId> = normWireObject<ChangeCriticalRegionBuildingId>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionBuildingId")), id: normWireRequired(normWireString), newBuildingId: normWireRequired(normWireString) });
