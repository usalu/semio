/** 🏗️ `change-bridge-span` wire twin: the leaf payload `ChangeBridgeSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeSpan {
  newBridgeSpan: number;
}

export const parseChangeBridgeSpan: NormWireReader<ChangeBridgeSpan> = normWireObject<ChangeBridgeSpan>({ newBridgeSpan: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
