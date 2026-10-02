/** 🚛 `change-assumed-bridge-lm3` wire twin: the leaf payload `ChangeAssumedBridgeLm3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeLm3 {
  newAssumedBridgeLm3: number;
}

export const parseChangeAssumedBridgeLm3: NormWireReader<ChangeAssumedBridgeLm3> = normWireObject<ChangeAssumedBridgeLm3>({ newAssumedBridgeLm3: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
