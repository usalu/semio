/** 🏗️ `change-is-basement` wire twin: the leaf payload `ChangeIsBasement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeIsBasement {
  index: number;
  newIsBasement: boolean;
}

export const parseChangeIsBasement: NormWireReader<ChangeIsBasement> = normWireObject<ChangeIsBasement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newIsBasement: normWireRequired(normWireBoolean) });
