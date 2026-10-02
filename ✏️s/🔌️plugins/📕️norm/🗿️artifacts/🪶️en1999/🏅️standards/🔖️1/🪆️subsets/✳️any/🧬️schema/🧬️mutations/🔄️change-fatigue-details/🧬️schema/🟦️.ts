/** 🔄️ `change-fatigue-details` wire twin: the leaf payload `ChangeFatigueDetails`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FatigueDetail, parseFatigueDetail } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFatigueDetails {
  mutation: "changeFatigueDetails";
  fatigueDetails: FatigueDetail[];
}

export const parseChangeFatigueDetails: NormWireReader<ChangeFatigueDetails> = normWireObject<ChangeFatigueDetails>({ mutation: normWireRequired(normWireLiteral("changeFatigueDetails")), fatigueDetails: normWireRequired(normWireArray(parseFatigueDetail)) });
