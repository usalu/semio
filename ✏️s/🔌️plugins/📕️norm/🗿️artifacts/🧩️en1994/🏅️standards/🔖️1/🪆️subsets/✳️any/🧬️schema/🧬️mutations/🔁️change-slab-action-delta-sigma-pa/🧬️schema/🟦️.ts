/** 🔁️ `change-slab-action-delta-sigma-pa` wire twin: the leaf payload `ChangeSlabActionDeltaSigmaPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabActionDeltaSigmaPa {
  slabId: string;
  id: string;
  newDeltaSigmaKPa: number;
}

export const parseChangeSlabActionDeltaSigmaPa: NormWireReader<ChangeSlabActionDeltaSigmaPa> = normWireObject<ChangeSlabActionDeltaSigmaPa>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDeltaSigmaKPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
