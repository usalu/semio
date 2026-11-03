/** 🛣️ `change-assumed-bridge-udl` wire twin: the leaf payload `ChangeAssumedBridgeUdl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedBridgeUdl {
  newAssumedBridgeUdl: number;
}

export const parseChangeAssumedBridgeUdl: NormWireReader<ChangeAssumedBridgeUdl> = normWireObject<ChangeAssumedBridgeUdl>({ newAssumedBridgeUdl: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
