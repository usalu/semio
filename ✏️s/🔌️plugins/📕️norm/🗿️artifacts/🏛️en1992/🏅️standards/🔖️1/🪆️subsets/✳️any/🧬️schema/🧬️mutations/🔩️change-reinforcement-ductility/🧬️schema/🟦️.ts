/** 🔩 `change-reinforcement-ductility` wire twin: the leaf payload `ChangeReinforcementDuctility`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DuctilityClass, parseDuctilityClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeReinforcementDuctility {
  gradeId: string;
  newDuctility: DuctilityClass;
}

export const parseChangeReinforcementDuctility: NormWireReader<ChangeReinforcementDuctility> = normWireObject<ChangeReinforcementDuctility>({ gradeId: normWireRequired(normWireString), newDuctility: normWireRequired(parseDuctilityClass) });
