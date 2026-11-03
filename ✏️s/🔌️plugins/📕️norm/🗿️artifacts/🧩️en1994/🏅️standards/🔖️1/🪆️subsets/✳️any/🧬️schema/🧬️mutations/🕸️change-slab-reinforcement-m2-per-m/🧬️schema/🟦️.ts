/** 🕸️ `change-slab-reinforcement-m2-per-m` wire twin: the leaf payload `ChangeSlabReinforcementM2PerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabReinforcementM2PerM {
  id: string;
  newAsM2PerM: number;
}

export const parseChangeSlabReinforcementM2PerM: NormWireReader<ChangeSlabReinforcementM2PerM> = normWireObject<ChangeSlabReinforcementM2PerM>({ id: normWireRequired(normWireString), newAsM2PerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
