/** 📐️ `change-slab-bearing-depth` wire twin: the leaf payload `ChangeSlabBearingDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabBearingDepth {
  id: string;
  newSlabBearingDepthM: number;
}

export const parseChangeSlabBearingDepth: NormWireReader<ChangeSlabBearingDepth> = normWireObject<ChangeSlabBearingDepth>({ id: normWireRequired(normWireString), newSlabBearingDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
