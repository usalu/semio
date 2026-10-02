/** 🪟 `change-opening-width` wire twin: the leaf payload `ChangeOpeningWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeOpeningWidth {
  wallIndex: number;
  index: number;
  newWidthM: number;
}

export const parseChangeOpeningWidth: NormWireReader<ChangeOpeningWidth> = normWireObject<ChangeOpeningWidth>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
