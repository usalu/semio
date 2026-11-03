/** 🎻 `change-prestress-eccentricity` wire twin: the leaf payload `ChangePrestressEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressEccentricity {
  wallId: string;
  id: string;
  newEccentricityM: number;
}

export const parseChangePrestressEccentricity: NormWireReader<ChangePrestressEccentricity> = normWireObject<ChangePrestressEccentricity>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newEccentricityM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
