/** ➖ `remove-slab` wire twin: the leaf payload `RemoveSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSlab {
  id: string;
}

export const parseRemoveSlab: NormWireReader<RemoveSlab> = normWireObject<RemoveSlab>({ id: normWireRequired(normWireString) });
