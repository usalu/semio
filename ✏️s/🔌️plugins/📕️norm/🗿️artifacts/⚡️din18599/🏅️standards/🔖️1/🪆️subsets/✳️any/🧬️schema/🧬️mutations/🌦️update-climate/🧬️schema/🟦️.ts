/** 🌦️ `update-climate` wire twin: the leaf payload `UpdateClimate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599MonthlyClimate, parseDin18599MonthlyClimate } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateClimate {
  mutation: "updateClimate";
  newClimate: Din18599MonthlyClimate;
}

export const parseUpdateClimate: NormWireReader<UpdateClimate> = normWireObject<UpdateClimate>({ mutation: normWireRequired(normWireLiteral("updateClimate")), newClimate: normWireRequired(parseDin18599MonthlyClimate) });
