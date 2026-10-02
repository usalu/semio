/** 🏷️ `change-imposed-category` wire twin: the leaf payload `ChangeImposedCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeImposedCategory {
  wallIndex: number;
  index: number;
  newImposedCategory: string;
}

export const parseChangeImposedCategory: NormWireReader<ChangeImposedCategory> = normWireObject<ChangeImposedCategory>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newImposedCategory: normWireRequired(normWireString) });
