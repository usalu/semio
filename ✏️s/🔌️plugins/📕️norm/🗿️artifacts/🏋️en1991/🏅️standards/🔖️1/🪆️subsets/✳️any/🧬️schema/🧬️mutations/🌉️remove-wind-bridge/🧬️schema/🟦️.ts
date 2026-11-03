/** 🌉 `remove-wind-bridge` wire twin: the leaf payload `RemoveWindBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveWindBridge {
  id: string;
}

export const parseRemoveWindBridge: NormWireReader<RemoveWindBridge> = normWireObject<RemoveWindBridge>({ id: normWireRequired(normWireString) });
