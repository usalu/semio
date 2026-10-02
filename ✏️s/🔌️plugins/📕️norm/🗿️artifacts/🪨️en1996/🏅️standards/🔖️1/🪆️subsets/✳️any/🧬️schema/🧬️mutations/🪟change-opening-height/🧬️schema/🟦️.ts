/** 🪟 `change-opening-height` wire twin: the leaf payload `ChangeOpeningHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeOpeningHeight {
  wallIndex: number;
  index: number;
  newHeightM: number;
}

export const parseChangeOpeningHeight: NormWireReader<ChangeOpeningHeight> = normWireObject<ChangeOpeningHeight>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
