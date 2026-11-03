/** 🧭️ `change-slab-sheeting-ribs-parallel` wire twin: the leaf payload `ChangeSlabSheetingRibsParallel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabSheetingRibsParallel {
  id: string;
  newRibsParallelToBeam: boolean;
}

export const parseChangeSlabSheetingRibsParallel: NormWireReader<ChangeSlabSheetingRibsParallel> = normWireObject<ChangeSlabSheetingRibsParallel>({ id: normWireRequired(normWireString), newRibsParallelToBeam: normWireRequired(normWireBoolean) });
