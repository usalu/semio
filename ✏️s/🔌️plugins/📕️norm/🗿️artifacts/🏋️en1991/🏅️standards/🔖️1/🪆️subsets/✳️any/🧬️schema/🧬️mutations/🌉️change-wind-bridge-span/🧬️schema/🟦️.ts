/** 🌉 `change-wind-bridge-span` wire twin: the leaf payload `ChangeWindBridgeSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindBridgeSpan {
  id: string;
  newSpan: number;
}

export const parseChangeWindBridgeSpan: NormWireReader<ChangeWindBridgeSpan> = normWireObject<ChangeWindBridgeSpan>({ id: normWireRequired(normWireString), newSpan: normWireRequired(normWireNumber) });
