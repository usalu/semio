/** 🛣️ `remove-bridge-lane` wire twin: the leaf payload `RemoveBridgeLane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBridgeLane {
  id: string;
}

export const parseRemoveBridgeLane: NormWireReader<RemoveBridgeLane> = normWireObject<RemoveBridgeLane>({ id: normWireRequired(normWireString) });
