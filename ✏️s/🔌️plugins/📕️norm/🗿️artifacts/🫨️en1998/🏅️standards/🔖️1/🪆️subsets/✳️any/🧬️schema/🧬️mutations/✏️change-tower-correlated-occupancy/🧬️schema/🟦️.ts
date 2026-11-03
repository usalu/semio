/** ✏️ `change-tower-correlated-occupancy` wire twin: the leaf payload `ChangeTowerCorrelatedOccupancy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerCorrelatedOccupancy {
  mutation: "changeTowerCorrelatedOccupancy";
  id: string;
  newCorrelatedOccupancy: boolean;
}

export const parseChangeTowerCorrelatedOccupancy: NormWireReader<ChangeTowerCorrelatedOccupancy> = normWireObject<ChangeTowerCorrelatedOccupancy>({ mutation: normWireRequired(normWireLiteral("changeTowerCorrelatedOccupancy")), id: normWireRequired(normWireString), newCorrelatedOccupancy: normWireRequired(normWireBoolean) });
