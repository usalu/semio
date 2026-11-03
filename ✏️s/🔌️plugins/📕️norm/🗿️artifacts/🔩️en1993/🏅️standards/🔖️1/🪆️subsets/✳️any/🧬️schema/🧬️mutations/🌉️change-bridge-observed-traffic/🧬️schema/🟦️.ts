/** 🌉️ `change-bridge-observed-traffic` wire twin: the leaf payload `ChangeBridgeObservedTraffic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeObservedTraffic {
  id: string;
  newObservedTraffic: number;
}

export const parseChangeBridgeObservedTraffic: NormWireReader<ChangeBridgeObservedTraffic> = normWireObject<ChangeBridgeObservedTraffic>({ id: normWireRequired(normWireString), newObservedTraffic: normWireRequired(normWireNumber) });
