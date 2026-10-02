/** 🔩 `change-as-vertical` wire twin: the leaf payload `ChangeAsVertical`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAsVertical {
  index: number;
  newAsVerticalM2: number;
}

export const parseChangeAsVertical: NormWireReader<ChangeAsVertical> = normWireObject<ChangeAsVertical>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAsVerticalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
