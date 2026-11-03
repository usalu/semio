/** 🌉 `change-wind-bridge-assumed-force` wire twin: the leaf payload `ChangeWindBridgeAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeWindBridgeAssumedForce: NormWireReader<ChangeWindBridgeAssumedForce> = normWireObject<ChangeWindBridgeAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireNumber) });
