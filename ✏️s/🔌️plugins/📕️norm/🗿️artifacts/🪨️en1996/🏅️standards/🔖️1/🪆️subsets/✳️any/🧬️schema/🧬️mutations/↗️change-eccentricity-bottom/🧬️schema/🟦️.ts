/** ↗️ `change-eccentricity-bottom` wire twin: the leaf payload `ChangeEccentricityBottom`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEccentricityBottom {
  id: string;
  newEccentricityBottomM: number;
}

export const parseChangeEccentricityBottom: NormWireReader<ChangeEccentricityBottom> = normWireObject<ChangeEccentricityBottom>({ id: normWireRequired(normWireString), newEccentricityBottomM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
