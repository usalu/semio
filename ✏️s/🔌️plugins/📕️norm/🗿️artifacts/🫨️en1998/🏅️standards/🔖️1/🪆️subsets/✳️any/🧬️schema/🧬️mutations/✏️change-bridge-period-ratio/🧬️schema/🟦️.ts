/** ✏️ `change-bridge-period-ratio` wire twin: the leaf payload `ChangeBridgePeriodRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePeriodRatio {
  mutation: "changeBridgePeriodRatio";
  id: string;
  newPeriodRatio: number;
}

export const parseChangeBridgePeriodRatio: NormWireReader<ChangeBridgePeriodRatio> = normWireObject<ChangeBridgePeriodRatio>({ mutation: normWireRequired(normWireLiteral("changeBridgePeriodRatio")), id: normWireRequired(normWireString), newPeriodRatio: normWireRequired(normWireNumber) });
