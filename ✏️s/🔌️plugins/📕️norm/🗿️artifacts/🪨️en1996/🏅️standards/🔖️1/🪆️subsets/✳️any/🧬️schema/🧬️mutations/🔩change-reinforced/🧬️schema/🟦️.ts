/** 🔩 `change-reinforced` wire twin: the leaf payload `ChangeReinforced`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeReinforced {
  index: number;
  newReinforced: boolean;
}

export const parseChangeReinforced: NormWireReader<ChangeReinforced> = normWireObject<ChangeReinforced>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newReinforced: normWireRequired(normWireBoolean) });
