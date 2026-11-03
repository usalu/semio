/** 🔄 `change-in-plane-moment` wire twin: the leaf payload `ChangeInPlaneMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeInPlaneMoment {
  id: string;
  newInPlaneMomentNm: number;
}

export const parseChangeInPlaneMoment: NormWireReader<ChangeInPlaneMoment> = normWireObject<ChangeInPlaneMoment>({ id: normWireRequired(normWireString), newInPlaneMomentNm: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000000000000,"maximum":10000000000000})) });
