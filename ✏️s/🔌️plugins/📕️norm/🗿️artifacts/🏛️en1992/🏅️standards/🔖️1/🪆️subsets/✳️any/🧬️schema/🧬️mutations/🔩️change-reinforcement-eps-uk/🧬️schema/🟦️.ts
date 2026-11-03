/** 🔩 `change-reinforcement-eps-uk` wire twin: the leaf payload `ChangeReinforcementEpsUk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforcementEpsUk {
  gradeId: string;
  newEpsUk: number;
}

export const parseChangeReinforcementEpsUk: NormWireReader<ChangeReinforcementEpsUk> = normWireObject<ChangeReinforcementEpsUk>({ gradeId: normWireRequired(normWireString), newEpsUk: normWireRequired(normWireNumber) });
