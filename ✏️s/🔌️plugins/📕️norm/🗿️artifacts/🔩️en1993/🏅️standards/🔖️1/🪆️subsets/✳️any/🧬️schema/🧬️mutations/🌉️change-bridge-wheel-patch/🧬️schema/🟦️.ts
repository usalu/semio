/** 🌉️ `change-bridge-wheel-patch` wire twin: the leaf payload `ChangeBridgeWheelPatch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeWheelPatch {
  id: string;
  newWheelPatch: number;
}

export const parseChangeBridgeWheelPatch: NormWireReader<ChangeBridgeWheelPatch> = normWireObject<ChangeBridgeWheelPatch>({ id: normWireRequired(normWireString), newWheelPatch: normWireRequired(normWireNumber) });
