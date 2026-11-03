/** ✏️ `change-bridge-correlated-occupancy` wire twin: the leaf payload `ChangeBridgeCorrelatedOccupancy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeCorrelatedOccupancy {
  mutation: "changeBridgeCorrelatedOccupancy";
  id: string;
  newCorrelatedOccupancy: boolean;
}

export const parseChangeBridgeCorrelatedOccupancy: NormWireReader<ChangeBridgeCorrelatedOccupancy> = normWireObject<ChangeBridgeCorrelatedOccupancy>({ mutation: normWireRequired(normWireLiteral("changeBridgeCorrelatedOccupancy")), id: normWireRequired(normWireString), newCorrelatedOccupancy: normWireRequired(normWireBoolean) });
