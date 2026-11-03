/** ↘️ `change-eccentricity-top` wire twin: the leaf payload `ChangeEccentricityTop`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEccentricityTop {
  id: string;
  newEccentricityTopM: number;
}

export const parseChangeEccentricityTop: NormWireReader<ChangeEccentricityTop> = normWireObject<ChangeEccentricityTop>({ id: normWireRequired(normWireString), newEccentricityTopM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
