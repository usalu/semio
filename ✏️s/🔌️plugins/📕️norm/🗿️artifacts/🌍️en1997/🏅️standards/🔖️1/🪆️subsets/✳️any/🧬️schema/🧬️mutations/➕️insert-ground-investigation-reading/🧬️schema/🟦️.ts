/** ➕ `insert-ground-investigation-reading` wire twin: the leaf payload `InsertGroundInvestigationReading`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997InvestigationReading, parseEn1997InvestigationReading } from "../../../📸️snapshot/🟦️.ts";

export interface InsertGroundInvestigationReading {
  mutation: "insertGroundInvestigationReading";
  investigationId: string;
  index: number;
  investigationReading: En1997InvestigationReading;
}

export const parseInsertGroundInvestigationReading: NormWireReader<InsertGroundInvestigationReading> = normWireObject<InsertGroundInvestigationReading>({ mutation: normWireRequired(normWireLiteral("insertGroundInvestigationReading")), investigationId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), investigationReading: normWireRequired(parseEn1997InvestigationReading) });
