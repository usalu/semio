/** ➖️ `remove-lintel` wire twin: the leaf payload `RemoveLintel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLintel {
  wallId: string;
  id: string;
}

export const parseRemoveLintel: NormWireReader<RemoveLintel> = normWireObject<RemoveLintel>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
