/** 🌉️ `change-bridge-detail-id` wire twin: the leaf payload `ChangeBridgeDetailId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDetailId {
  id: string;
  newDetailId: string;
}

export const parseChangeBridgeDetailId: NormWireReader<ChangeBridgeDetailId> = normWireObject<ChangeBridgeDetailId>({ id: normWireRequired(normWireString), newDetailId: normWireRequired(normWireString) });
