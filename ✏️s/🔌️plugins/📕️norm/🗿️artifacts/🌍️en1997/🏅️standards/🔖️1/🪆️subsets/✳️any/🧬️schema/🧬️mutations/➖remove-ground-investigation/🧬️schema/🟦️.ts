/** ➖ `remove-ground-investigation` wire twin: the leaf payload `RemoveGroundInvestigation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveGroundInvestigation {
  mutation: "removeGroundInvestigation";
  index: number;
}

export const parseRemoveGroundInvestigation: NormWireReader<RemoveGroundInvestigation> = normWireObject<RemoveGroundInvestigation>({ mutation: normWireRequired(normWireLiteral("removeGroundInvestigation")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
