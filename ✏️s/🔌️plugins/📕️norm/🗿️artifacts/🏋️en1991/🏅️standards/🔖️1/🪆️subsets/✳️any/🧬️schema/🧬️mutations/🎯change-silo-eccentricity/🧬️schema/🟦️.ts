/** 🎯 `change-silo-eccentricity` wire twin: the leaf payload `ChangeSiloEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloEccentricity {
  newSiloEccentricity: number;
}

export const parseChangeSiloEccentricity: NormWireReader<ChangeSiloEccentricity> = normWireObject<ChangeSiloEccentricity>({ newSiloEccentricity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
