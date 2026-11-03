/** ➖️ `remove-tie` wire twin: the leaf payload `RemoveTie`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTie {
  wallId: string;
  id: string;
}

export const parseRemoveTie: NormWireReader<RemoveTie> = normWireObject<RemoveTie>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
