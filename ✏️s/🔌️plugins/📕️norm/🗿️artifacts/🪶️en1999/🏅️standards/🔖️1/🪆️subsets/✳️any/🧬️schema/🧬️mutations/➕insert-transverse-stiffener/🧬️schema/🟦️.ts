/** 🗂️ `insert-transverse-stiffener` wire twin: the leaf payload `InsertTransverseStiffener`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TransverseStiffener, parseTransverseStiffener } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTransverseStiffener {
  mutation: "insertTransverseStiffener";
  index: number;
  transverseStiffener: TransverseStiffener;
}

export const parseInsertTransverseStiffener: NormWireReader<InsertTransverseStiffener> = normWireObject<InsertTransverseStiffener>({ mutation: normWireRequired(normWireLiteral("insertTransverseStiffener")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), transverseStiffener: normWireRequired(parseTransverseStiffener) });
