/** ✏️ `change-isolation-system-building-id` wire twin: the leaf payload `ChangeIsolationSystemBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemBuildingId {
  mutation: "changeIsolationSystemBuildingId";
  id: string;
  newBuildingId: string;
}

export const parseChangeIsolationSystemBuildingId: NormWireReader<ChangeIsolationSystemBuildingId> = normWireObject<ChangeIsolationSystemBuildingId>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemBuildingId")), id: normWireRequired(normWireString), newBuildingId: normWireRequired(normWireString) });
