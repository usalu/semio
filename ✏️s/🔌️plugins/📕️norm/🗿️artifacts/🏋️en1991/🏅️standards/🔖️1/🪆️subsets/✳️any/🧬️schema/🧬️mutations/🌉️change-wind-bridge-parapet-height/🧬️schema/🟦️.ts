/** 🌉 `change-wind-bridge-parapet-height` wire twin: the leaf payload `ChangeWindBridgeParapetHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeParapetHeight {
  id: string;
  newParapetHeight: number;
}

export const parseChangeWindBridgeParapetHeight: NormWireReader<ChangeWindBridgeParapetHeight> = normWireObject<ChangeWindBridgeParapetHeight>({ id: normWireRequired(normWireString), newParapetHeight: normWireRequired(normWireNumber) });
