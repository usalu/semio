/** 🏗 `change-bridge-span` wire twin: the leaf payload `ChangeBridgeSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBridgeSpan {
  newBridgeSpan: number;
}

export const parseChangeBridgeSpan: NormWireReader<ChangeBridgeSpan> = normWireObject<ChangeBridgeSpan>({ newBridgeSpan: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
