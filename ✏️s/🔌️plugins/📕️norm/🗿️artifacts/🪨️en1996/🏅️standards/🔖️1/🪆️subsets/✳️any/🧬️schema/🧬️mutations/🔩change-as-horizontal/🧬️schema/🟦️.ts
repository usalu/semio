/** 🔩 `change-as-horizontal` wire twin: the leaf payload `ChangeAsHorizontal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAsHorizontal {
  index: number;
  newAsHorizontalM2: number;
}

export const parseChangeAsHorizontal: NormWireReader<ChangeAsHorizontal> = normWireObject<ChangeAsHorizontal>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAsHorizontalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
