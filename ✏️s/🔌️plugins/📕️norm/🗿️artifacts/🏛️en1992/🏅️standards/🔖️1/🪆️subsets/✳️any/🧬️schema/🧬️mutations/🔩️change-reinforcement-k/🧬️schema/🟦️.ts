/** 🔩 `change-reinforcement-k` wire twin: the leaf payload `ChangeReinforcementK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforcementK {
  gradeId: string;
  newK: number;
}

export const parseChangeReinforcementK: NormWireReader<ChangeReinforcementK> = normWireObject<ChangeReinforcementK>({ gradeId: normWireRequired(normWireString), newK: normWireRequired(normWireNumber) });
