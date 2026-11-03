/** 🧭 `change-plan-y` wire twin: the leaf payload `ChangePlanY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlanY {
  id: string;
  newPlanYM: number;
}

export const parseChangePlanY: NormWireReader<ChangePlanY> = normWireObject<ChangePlanY>({ id: normWireRequired(normWireString), newPlanYM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
