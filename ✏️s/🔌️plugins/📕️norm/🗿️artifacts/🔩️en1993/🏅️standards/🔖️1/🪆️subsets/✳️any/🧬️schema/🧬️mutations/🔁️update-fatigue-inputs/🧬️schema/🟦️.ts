/** 🔁️ `update-fatigue-inputs` wire twin: the leaf payload `UpdateFatigueInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FatigueDetail, parseFatigueDetail } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateFatigueInputs {
  fatigueDetail: FatigueDetail;
}

export const parseUpdateFatigueInputs: NormWireReader<UpdateFatigueInputs> = normWireObject<UpdateFatigueInputs>({ fatigueDetail: normWireRequired(parseFatigueDetail) });
