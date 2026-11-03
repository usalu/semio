/** 🧰️ `change-confined-depth` wire twin: the leaf payload `ChangeConfinedDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConfinedDepth {
  wallId: string;
  id: string;
  newDepthM: number;
}

export const parseChangeConfinedDepth: NormWireReader<ChangeConfinedDepth> = normWireObject<ChangeConfinedDepth>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
