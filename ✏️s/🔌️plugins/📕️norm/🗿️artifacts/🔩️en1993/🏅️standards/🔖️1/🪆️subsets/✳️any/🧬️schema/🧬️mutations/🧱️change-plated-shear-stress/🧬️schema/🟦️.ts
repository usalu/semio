/** 🧱️ `change-plated-shear-stress` wire twin: the leaf payload `ChangePlatedShearStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedShearStress {
  id: string;
  newShearStress: number;
}

export const parseChangePlatedShearStress: NormWireReader<ChangePlatedShearStress> = normWireObject<ChangePlatedShearStress>({ id: normWireRequired(normWireString), newShearStress: normWireRequired(normWireNumber) });
