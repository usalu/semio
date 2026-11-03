/** 👥 `change-assumed-bridge-lm4` wire twin: the leaf payload `ChangeAssumedBridgeLm4`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedBridgeLm4 {
  newAssumedBridgeLm4: number;
}

export const parseChangeAssumedBridgeLm4: NormWireReader<ChangeAssumedBridgeLm4> = normWireObject<ChangeAssumedBridgeLm4>({ newAssumedBridgeLm4: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
