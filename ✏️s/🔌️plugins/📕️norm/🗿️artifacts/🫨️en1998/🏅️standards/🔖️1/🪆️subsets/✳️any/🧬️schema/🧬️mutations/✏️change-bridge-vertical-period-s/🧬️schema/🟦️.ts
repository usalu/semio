/** ✏️ `change-bridge-vertical-period-s` wire twin: the leaf payload `ChangeBridgeVerticalPeriodS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeVerticalPeriodS {
  mutation: "changeBridgeVerticalPeriodS";
  id: string;
  newVerticalPeriodS: number;
}

export const parseChangeBridgeVerticalPeriodS: NormWireReader<ChangeBridgeVerticalPeriodS> = normWireObject<ChangeBridgeVerticalPeriodS>({ mutation: normWireRequired(normWireLiteral("changeBridgeVerticalPeriodS")), id: normWireRequired(normWireString), newVerticalPeriodS: normWireRequired(normWireNumber) });
