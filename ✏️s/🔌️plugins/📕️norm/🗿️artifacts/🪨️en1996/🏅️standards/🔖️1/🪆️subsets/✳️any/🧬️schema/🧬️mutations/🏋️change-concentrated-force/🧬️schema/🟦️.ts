/** 🏋️ `change-concentrated-force` wire twin: the leaf payload `ChangeConcentratedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcentratedForce {
  wallId: string;
  loadCaseId: string;
  id: string;
  newForceN: number;
}

export const parseChangeConcentratedForce: NormWireReader<ChangeConcentratedForce> = normWireObject<ChangeConcentratedForce>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), id: normWireRequired(normWireString), newForceN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})) });
