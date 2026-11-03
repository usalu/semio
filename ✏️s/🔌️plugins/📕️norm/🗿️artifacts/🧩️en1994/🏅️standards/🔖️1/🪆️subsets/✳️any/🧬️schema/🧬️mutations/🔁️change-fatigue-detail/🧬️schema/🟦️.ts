/** 🔁️ `change-fatigue-detail` wire twin: the leaf payload `ChangeFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueDetail, parseFatigueDetail } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFatigueDetail {
  newFatigueDetail: FatigueDetail;
}

export const parseChangeFatigueDetail: NormWireReader<ChangeFatigueDetail> = normWireObject<ChangeFatigueDetail>({ newFatigueDetail: normWireRequired(parseFatigueDetail) });
