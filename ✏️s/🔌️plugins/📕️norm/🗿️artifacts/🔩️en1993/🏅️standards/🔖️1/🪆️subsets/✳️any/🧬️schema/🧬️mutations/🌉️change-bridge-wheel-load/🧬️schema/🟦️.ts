/** 🌉️ `change-bridge-wheel-load` wire twin: the leaf payload `ChangeBridgeWheelLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeWheelLoad {
  id: string;
  newWheelLoad: number;
}

export const parseChangeBridgeWheelLoad: NormWireReader<ChangeBridgeWheelLoad> = normWireObject<ChangeBridgeWheelLoad>({ id: normWireRequired(normWireString), newWheelLoad: normWireRequired(normWireNumber) });
