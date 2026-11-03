/** 🧱️ `change-plated-stiffener-eccentricity` wire twin: the leaf payload `ChangePlatedStiffenerEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerEccentricity {
  id: string;
  newStiffenerEccentricity: number;
}

export const parseChangePlatedStiffenerEccentricity: NormWireReader<ChangePlatedStiffenerEccentricity> = normWireObject<ChangePlatedStiffenerEccentricity>({ id: normWireRequired(normWireString), newStiffenerEccentricity: normWireRequired(normWireNumber) });
