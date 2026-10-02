/** 🚦️ `change-script-limits` wire twin: the leaf payload `ChangeScriptLimits`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeScriptLimits {
  newMaxSteps: number;
  newMaxRecursion: number;
  newTimeoutMs: number;
}

export const parseChangeScriptLimits: NormWireReader<ChangeScriptLimits> = normWireObject<ChangeScriptLimits>({ newMaxSteps: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMaxRecursion: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newTimeoutMs: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
