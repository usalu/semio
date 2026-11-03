/** ➕ `insert-ground-investigation` wire twin: the leaf payload `InsertGroundInvestigation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997Investigation, parseEn1997Investigation } from "../../../📸️snapshot/🟦️.ts";

export interface InsertGroundInvestigation {
  mutation: "insertGroundInvestigation";
  index: number;
  investigation: En1997Investigation;
}

export const parseInsertGroundInvestigation: NormWireReader<InsertGroundInvestigation> = normWireObject<InsertGroundInvestigation>({ mutation: normWireRequired(normWireLiteral("insertGroundInvestigation")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), investigation: normWireRequired(parseEn1997Investigation) });
