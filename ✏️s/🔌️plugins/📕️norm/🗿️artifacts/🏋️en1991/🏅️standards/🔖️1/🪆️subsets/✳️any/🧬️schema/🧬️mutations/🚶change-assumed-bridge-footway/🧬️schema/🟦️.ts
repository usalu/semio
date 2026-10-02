/** 🚶 `change-assumed-bridge-footway` wire twin: the leaf payload `ChangeAssumedBridgeFootway`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeFootway {
  newAssumedBridgeFootway: number;
}

export const parseChangeAssumedBridgeFootway: NormWireReader<ChangeAssumedBridgeFootway> = normWireObject<ChangeAssumedBridgeFootway>({ newAssumedBridgeFootway: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
