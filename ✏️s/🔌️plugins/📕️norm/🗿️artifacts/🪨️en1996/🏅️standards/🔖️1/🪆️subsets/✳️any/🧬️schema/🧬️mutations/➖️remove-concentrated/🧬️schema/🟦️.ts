/** ➖️ `remove-concentrated` wire twin: the leaf payload `RemoveConcentrated`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveConcentrated {
  wallIndex: number;
  loadCaseIndex: number;
  index: number;
}

export const parseRemoveConcentrated: NormWireReader<RemoveConcentrated> = normWireObject<RemoveConcentrated>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
