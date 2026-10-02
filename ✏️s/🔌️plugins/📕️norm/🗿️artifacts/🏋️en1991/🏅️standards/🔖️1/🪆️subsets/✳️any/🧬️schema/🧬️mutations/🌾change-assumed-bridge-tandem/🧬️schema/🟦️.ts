/** 🌾 `change-assumed-bridge-tandem` wire twin: the leaf payload `ChangeAssumedBridgeTandem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedBridgeTandem {
  newAssumedBridgeTandem: number;
}

export const parseChangeAssumedBridgeTandem: NormWireReader<ChangeAssumedBridgeTandem> = normWireObject<ChangeAssumedBridgeTandem>({ newAssumedBridgeTandem: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
