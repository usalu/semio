/** ✏️ `change-bridge-fundamental-period-s` wire twin: the leaf payload `ChangeBridgeFundamentalPeriodS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeFundamentalPeriodS {
  mutation: "changeBridgeFundamentalPeriodS";
  id: string;
  newFundamentalPeriodS: number;
}

export const parseChangeBridgeFundamentalPeriodS: NormWireReader<ChangeBridgeFundamentalPeriodS> = normWireObject<ChangeBridgeFundamentalPeriodS>({ mutation: normWireRequired(normWireLiteral("changeBridgeFundamentalPeriodS")), id: normWireRequired(normWireString), newFundamentalPeriodS: normWireRequired(normWireNumber) });
