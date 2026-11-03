/** 🗂️ `insert-fatigue-category` wire twin: the leaf payload `InsertFatigueCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueCategory, parseFatigueCategory } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueCategory {
  mutation: "insertFatigueCategory";
  index: number;
  fatigueCategory: FatigueCategory;
}

export const parseInsertFatigueCategory: NormWireReader<InsertFatigueCategory> = normWireObject<InsertFatigueCategory>({ mutation: normWireRequired(normWireLiteral("insertFatigueCategory")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fatigueCategory: normWireRequired(parseFatigueCategory) });
