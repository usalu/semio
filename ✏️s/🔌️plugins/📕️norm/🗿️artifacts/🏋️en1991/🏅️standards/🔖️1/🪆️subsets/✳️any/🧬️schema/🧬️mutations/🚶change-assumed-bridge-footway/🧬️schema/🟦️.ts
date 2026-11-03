/** 🚶 `change-assumed-bridge-footway` wire twin: the leaf payload `ChangeAssumedBridgeFootway`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedBridgeFootway {
  newAssumedBridgeFootway: number;
}

export const parseChangeAssumedBridgeFootway: NormWireReader<ChangeAssumedBridgeFootway> = normWireObject<ChangeAssumedBridgeFootway>({ newAssumedBridgeFootway: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
