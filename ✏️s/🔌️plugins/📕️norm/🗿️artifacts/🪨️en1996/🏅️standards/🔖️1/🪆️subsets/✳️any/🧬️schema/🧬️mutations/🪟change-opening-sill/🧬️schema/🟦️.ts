/** 🪟 `change-opening-sill` wire twin: the leaf payload `ChangeOpeningSill`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeOpeningSill {
  wallIndex: number;
  index: number;
  newSillHeightM: number;
}

export const parseChangeOpeningSill: NormWireReader<ChangeOpeningSill> = normWireObject<ChangeOpeningSill>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSillHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
