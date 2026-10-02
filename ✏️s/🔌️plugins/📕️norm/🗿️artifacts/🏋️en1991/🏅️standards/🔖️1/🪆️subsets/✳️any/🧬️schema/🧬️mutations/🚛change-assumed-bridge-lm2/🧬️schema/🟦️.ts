/** 🚛 `change-assumed-bridge-lm2` wire twin: the leaf payload `ChangeAssumedBridgeLm2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeLm2 {
  newAssumedBridgeLm2: number;
}

export const parseChangeAssumedBridgeLm2: NormWireReader<ChangeAssumedBridgeLm2> = normWireObject<ChangeAssumedBridgeLm2>({ newAssumedBridgeLm2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
