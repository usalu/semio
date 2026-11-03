/** ➖️ `remove-bridge-fatigue` wire twin: the leaf payload `RemoveBridgeFatigue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBridgeFatigue {
  index: number;
}

export const parseRemoveBridgeFatigue: NormWireReader<RemoveBridgeFatigue> = normWireObject<RemoveBridgeFatigue>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
