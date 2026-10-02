/** ➖️ `remove-bridge-fatigue` wire twin: the leaf payload `RemoveBridgeFatigue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveBridgeFatigue {
  index: number;
}

export const parseRemoveBridgeFatigue: NormWireReader<RemoveBridgeFatigue> = normWireObject<RemoveBridgeFatigue>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
