/** 📐️ `change-plan-angle` wire twin: the leaf payload `ChangePlanAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlanAngle {
  id: string;
  newPlanAngleDeg: number;
}

export const parseChangePlanAngle: NormWireReader<ChangePlanAngle> = normWireObject<ChangePlanAngle>({ id: normWireRequired(normWireString), newPlanAngleDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":-180,"maximum":180})) });
