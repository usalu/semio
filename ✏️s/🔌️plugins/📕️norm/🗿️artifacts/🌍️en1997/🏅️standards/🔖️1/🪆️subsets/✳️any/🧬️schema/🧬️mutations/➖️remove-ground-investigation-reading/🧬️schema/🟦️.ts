/** ➖ `remove-ground-investigation-reading` wire twin: the leaf payload `RemoveGroundInvestigationReading`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveGroundInvestigationReading {
  mutation: "removeGroundInvestigationReading";
  investigationId: string;
  index: number;
}

export const parseRemoveGroundInvestigationReading: NormWireReader<RemoveGroundInvestigationReading> = normWireObject<RemoveGroundInvestigationReading>({ mutation: normWireRequired(normWireLiteral("removeGroundInvestigationReading")), investigationId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
