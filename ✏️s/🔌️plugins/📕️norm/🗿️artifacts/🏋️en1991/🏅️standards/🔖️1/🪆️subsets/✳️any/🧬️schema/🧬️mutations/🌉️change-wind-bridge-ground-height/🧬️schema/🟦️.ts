/** 🌉 `change-wind-bridge-ground-height` wire twin: the leaf payload `ChangeWindBridgeGroundHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeGroundHeight {
  id: string;
  newHeightAboveGround: number;
}

export const parseChangeWindBridgeGroundHeight: NormWireReader<ChangeWindBridgeGroundHeight> = normWireObject<ChangeWindBridgeGroundHeight>({ id: normWireRequired(normWireString), newHeightAboveGround: normWireRequired(normWireNumber) });
