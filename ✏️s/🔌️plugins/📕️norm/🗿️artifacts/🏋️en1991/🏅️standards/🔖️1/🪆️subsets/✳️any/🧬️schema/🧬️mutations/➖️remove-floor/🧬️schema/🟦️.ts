/** ➖️ `remove-floor` wire twin: the leaf payload `RemoveFloor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFloor {
  id: string;
}

export const parseRemoveFloor: NormWireReader<RemoveFloor> = normWireObject<RemoveFloor>({ id: normWireRequired(normWireString) });
