/** 🛣️ `change-assumed-bridge-udl` wire twin: the leaf payload `ChangeAssumedBridgeUdl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeUdl {
  newAssumedBridgeUdl: number;
}

export const parseChangeAssumedBridgeUdl: NormWireReader<ChangeAssumedBridgeUdl> = normWireObject<ChangeAssumedBridgeUdl>({ newAssumedBridgeUdl: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
