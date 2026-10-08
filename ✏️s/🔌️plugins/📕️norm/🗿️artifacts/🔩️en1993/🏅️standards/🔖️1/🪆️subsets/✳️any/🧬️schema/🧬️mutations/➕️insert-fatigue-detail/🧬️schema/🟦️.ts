/** ➕️ `insert-fatigue-detail` wire twin: the leaf payload `InsertFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FatigueDetail, parseFatigueDetail } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueDetail {
  index?: number | null;
  fatigueDetail: FatigueDetail;
}

export const parseInsertFatigueDetail: NormWireReader<InsertFatigueDetail> = normWireObject<InsertFatigueDetail>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), fatigueDetail: normWireRequired(parseFatigueDetail) });
