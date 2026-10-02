/** 🔩 `change-reinforcement-f-yk` wire twin: the leaf payload `ChangeReinforcementFYk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeReinforcementFYk {
  gradeId: string;
  newFYk: number;
}

export const parseChangeReinforcementFYk: NormWireReader<ChangeReinforcementFYk> = normWireObject<ChangeReinforcementFYk>({ gradeId: normWireRequired(normWireString), newFYk: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
