/** 🗂️ `change-geotechnical-category` wire twin: the leaf payload `ChangeGeotechnicalCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeGeotechnicalCategory {
  mutation: "changeGeotechnicalCategory";
  newGeotechnicalCategory: number;
}

export const parseChangeGeotechnicalCategory: NormWireReader<ChangeGeotechnicalCategory> = normWireObject<ChangeGeotechnicalCategory>({ mutation: normWireRequired(normWireLiteral("changeGeotechnicalCategory")), newGeotechnicalCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":3})) });
