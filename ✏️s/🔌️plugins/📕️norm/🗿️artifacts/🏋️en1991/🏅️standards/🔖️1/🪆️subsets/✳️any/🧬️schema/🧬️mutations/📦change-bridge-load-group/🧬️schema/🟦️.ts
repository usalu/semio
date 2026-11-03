/** 📦 `change-bridge-load-group` wire twin: the leaf payload `ChangeBridgeLoadGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLoadGroup {
  newBridgeLoadGroup: string;
}

export const parseChangeBridgeLoadGroup: NormWireReader<ChangeBridgeLoadGroup> = normWireObject<ChangeBridgeLoadGroup>({ newBridgeLoadGroup: normWireRequired(normWireString) });
