/** ✏️ `change-storey-correlated-occupancy` wire twin: the leaf payload `ChangeStoreyCorrelatedOccupancy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCorrelatedOccupancy {
  mutation: "changeStoreyCorrelatedOccupancy";
  buildingId: string;
  id: string;
  newCorrelatedOccupancy: boolean;
}

export const parseChangeStoreyCorrelatedOccupancy: NormWireReader<ChangeStoreyCorrelatedOccupancy> = normWireObject<ChangeStoreyCorrelatedOccupancy>({ mutation: normWireRequired(normWireLiteral("changeStoreyCorrelatedOccupancy")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCorrelatedOccupancy: normWireRequired(normWireBoolean) });
