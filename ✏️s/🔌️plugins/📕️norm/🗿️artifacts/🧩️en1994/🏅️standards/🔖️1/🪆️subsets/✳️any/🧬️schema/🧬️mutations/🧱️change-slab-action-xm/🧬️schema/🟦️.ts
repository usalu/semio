/** 🧱️ `change-slab-action-xm` wire twin: the leaf payload `ChangeSlabActionXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabActionXM {
  slabId: string;
  id: string;
  newXM: number;
}

export const parseChangeSlabActionXM: NormWireReader<ChangeSlabActionXM> = normWireObject<ChangeSlabActionXM>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newXM: normWireRequired(normWireNumber) });
