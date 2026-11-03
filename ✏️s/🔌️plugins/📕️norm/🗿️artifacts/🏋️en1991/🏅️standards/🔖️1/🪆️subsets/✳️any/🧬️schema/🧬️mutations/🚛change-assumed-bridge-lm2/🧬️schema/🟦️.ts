/** 🚛 `change-assumed-bridge-lm2` wire twin: the leaf payload `ChangeAssumedBridgeLm2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedBridgeLm2 {
  newAssumedBridgeLm2: number;
}

export const parseChangeAssumedBridgeLm2: NormWireReader<ChangeAssumedBridgeLm2> = normWireObject<ChangeAssumedBridgeLm2>({ newAssumedBridgeLm2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
