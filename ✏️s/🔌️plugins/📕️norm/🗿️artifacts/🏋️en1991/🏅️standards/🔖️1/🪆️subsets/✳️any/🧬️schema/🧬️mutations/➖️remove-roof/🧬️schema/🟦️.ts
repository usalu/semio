/** ➖️ `remove-roof` wire twin: the leaf payload `RemoveRoof`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRoof {
  id: string;
}

export const parseRemoveRoof: NormWireReader<RemoveRoof> = normWireObject<RemoveRoof>({ id: normWireRequired(normWireString) });
