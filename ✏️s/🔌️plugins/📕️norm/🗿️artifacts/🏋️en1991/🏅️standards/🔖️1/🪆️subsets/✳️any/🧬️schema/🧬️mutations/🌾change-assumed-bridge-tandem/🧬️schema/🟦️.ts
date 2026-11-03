/** 🌾 `change-assumed-bridge-tandem` wire twin: the leaf payload `ChangeAssumedBridgeTandem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedBridgeTandem {
  newAssumedBridgeTandem: number;
}

export const parseChangeAssumedBridgeTandem: NormWireReader<ChangeAssumedBridgeTandem> = normWireObject<ChangeAssumedBridgeTandem>({ newAssumedBridgeTandem: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
