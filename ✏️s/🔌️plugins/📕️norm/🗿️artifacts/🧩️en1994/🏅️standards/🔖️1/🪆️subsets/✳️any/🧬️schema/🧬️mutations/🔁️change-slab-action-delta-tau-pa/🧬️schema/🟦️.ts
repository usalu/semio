/** 🔁️ `change-slab-action-delta-tau-pa` wire twin: the leaf payload `ChangeSlabActionDeltaTauPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabActionDeltaTauPa {
  slabId: string;
  id: string;
  newDeltaTauKPa: number;
}

export const parseChangeSlabActionDeltaTauPa: NormWireReader<ChangeSlabActionDeltaTauPa> = normWireObject<ChangeSlabActionDeltaTauPa>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDeltaTauKPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
