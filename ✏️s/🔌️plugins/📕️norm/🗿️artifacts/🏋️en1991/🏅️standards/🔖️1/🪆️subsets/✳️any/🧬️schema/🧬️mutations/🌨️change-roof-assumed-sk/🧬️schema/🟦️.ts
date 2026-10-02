/** 🌨️ `change-roof-assumed-sk` wire twin: the leaf payload `ChangeRoofAssumedSk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeRoofAssumedSk {
  index: number;
  newAssumedSk: number;
}

export const parseChangeRoofAssumedSk: NormWireReader<ChangeRoofAssumedSk> = normWireObject<ChangeRoofAssumedSk>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAssumedSk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
