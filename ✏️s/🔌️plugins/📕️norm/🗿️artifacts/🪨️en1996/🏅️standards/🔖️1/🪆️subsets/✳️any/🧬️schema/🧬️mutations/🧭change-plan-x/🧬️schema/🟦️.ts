/** 🧭 `change-plan-x` wire twin: the leaf payload `ChangePlanX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlanX {
  id: string;
  newPlanXM: number;
}

export const parseChangePlanX: NormWireReader<ChangePlanX> = normWireObject<ChangePlanX>({ id: normWireRequired(normWireString), newPlanXM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
