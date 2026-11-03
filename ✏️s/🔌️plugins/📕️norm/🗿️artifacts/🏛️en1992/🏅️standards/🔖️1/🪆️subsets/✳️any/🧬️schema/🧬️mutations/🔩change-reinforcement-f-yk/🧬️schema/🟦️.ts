/** 🔩 `change-reinforcement-f-yk` wire twin: the leaf payload `ChangeReinforcementFYk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforcementFYk {
  gradeId: string;
  newFYk: number;
}

export const parseChangeReinforcementFYk: NormWireReader<ChangeReinforcementFYk> = normWireObject<ChangeReinforcementFYk>({ gradeId: normWireRequired(normWireString), newFYk: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
