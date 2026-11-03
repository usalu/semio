/** 🔩 `change-reinforcement-es` wire twin: the leaf payload `ChangeReinforcementES`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforcementES {
  gradeId: string;
  newES: number;
}

export const parseChangeReinforcementES: NormWireReader<ChangeReinforcementES> = normWireObject<ChangeReinforcementES>({ gradeId: normWireRequired(normWireString), newES: normWireRequired(normWireNumber) });
