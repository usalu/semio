/** 🗂️ `insert-cold-formed-sheet` wire twin: the leaf payload `InsertColdFormedSheet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ColdFormedSheet, parseColdFormedSheet } from "../../../📸️snapshot/🟦️.ts";

export interface InsertColdFormedSheet {
  mutation: "insertColdFormedSheet";
  index: number;
  coldFormedSheet: ColdFormedSheet;
}

export const parseInsertColdFormedSheet: NormWireReader<InsertColdFormedSheet> = normWireObject<InsertColdFormedSheet>({ mutation: normWireRequired(normWireLiteral("insertColdFormedSheet")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), coldFormedSheet: normWireRequired(parseColdFormedSheet) });
