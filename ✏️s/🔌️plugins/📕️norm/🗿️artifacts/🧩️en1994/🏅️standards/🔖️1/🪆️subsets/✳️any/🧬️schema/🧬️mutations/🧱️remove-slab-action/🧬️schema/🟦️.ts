/** 🧱️ `remove-slab-action` wire twin: the leaf payload `RemoveSlabAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSlabAction {
  slabId: string;
  id: string;
}

export const parseRemoveSlabAction: NormWireReader<RemoveSlabAction> = normWireObject<RemoveSlabAction>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
