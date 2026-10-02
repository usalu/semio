/** 👥 `change-assumed-bridge-lm4` wire twin: the leaf payload `ChangeAssumedBridgeLm4`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeLm4 {
  newAssumedBridgeLm4: number;
}

export const parseChangeAssumedBridgeLm4: NormWireReader<ChangeAssumedBridgeLm4> = normWireObject<ChangeAssumedBridgeLm4>({ newAssumedBridgeLm4: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
