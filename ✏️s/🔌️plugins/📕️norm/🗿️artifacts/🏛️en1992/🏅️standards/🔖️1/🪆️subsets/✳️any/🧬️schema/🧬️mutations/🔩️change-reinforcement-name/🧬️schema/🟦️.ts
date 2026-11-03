/** 🔩 `change-reinforcement-name` wire twin: the leaf payload `ChangeReinforcementName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReinforcementName {
  gradeId: string;
  newName: string;
}

export const parseChangeReinforcementName: NormWireReader<ChangeReinforcementName> = normWireObject<ChangeReinforcementName>({ gradeId: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
