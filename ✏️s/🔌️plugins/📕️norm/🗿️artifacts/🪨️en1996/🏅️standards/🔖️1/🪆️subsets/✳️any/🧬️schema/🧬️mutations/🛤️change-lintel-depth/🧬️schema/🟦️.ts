/** 🛤️ `change-lintel-depth` wire twin: the leaf payload `ChangeLintelDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLintelDepth {
  wallId: string;
  id: string;
  newDepthM: number;
}

export const parseChangeLintelDepth: NormWireReader<ChangeLintelDepth> = normWireObject<ChangeLintelDepth>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
