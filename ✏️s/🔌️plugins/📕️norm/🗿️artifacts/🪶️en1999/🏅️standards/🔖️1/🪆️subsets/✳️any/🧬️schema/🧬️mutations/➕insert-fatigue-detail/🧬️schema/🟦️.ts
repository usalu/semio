/** 🗂️ `insert-fatigue-detail` wire twin: the leaf payload `InsertFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueDetail, parseFatigueDetail } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueDetail {
  mutation: "insertFatigueDetail";
  index: number;
  fatigueDetail: FatigueDetail;
}

export const parseInsertFatigueDetail: NormWireReader<InsertFatigueDetail> = normWireObject<InsertFatigueDetail>({ mutation: normWireRequired(normWireLiteral("insertFatigueDetail")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fatigueDetail: normWireRequired(parseFatigueDetail) });
