/** 📦 `change-bridge-load-group` wire twin: the leaf payload `ChangeBridgeLoadGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBridgeLoadGroup {
  newBridgeLoadGroup: string;
}

export const parseChangeBridgeLoadGroup: NormWireReader<ChangeBridgeLoadGroup> = normWireObject<ChangeBridgeLoadGroup>({ newBridgeLoadGroup: normWireRequired(normWireString) });
