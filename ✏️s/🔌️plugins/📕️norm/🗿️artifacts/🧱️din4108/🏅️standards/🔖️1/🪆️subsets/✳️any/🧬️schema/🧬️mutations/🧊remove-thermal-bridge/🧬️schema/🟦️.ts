/** 🧊 `remove-thermal-bridge` wire twin: the leaf payload `RemoveThermalBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveThermalBridge {
  index: number;
}

export const parseRemoveThermalBridge: NormWireReader<RemoveThermalBridge> = normWireObject<RemoveThermalBridge>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
