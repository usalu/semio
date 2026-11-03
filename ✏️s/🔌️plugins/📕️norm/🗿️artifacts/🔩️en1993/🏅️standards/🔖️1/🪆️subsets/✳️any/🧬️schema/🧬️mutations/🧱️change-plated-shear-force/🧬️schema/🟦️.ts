/** 🧱️ `change-plated-shear-force` wire twin: the leaf payload `ChangePlatedShearForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedShearForce {
  id: string;
  newShearForce: number;
}

export const parseChangePlatedShearForce: NormWireReader<ChangePlatedShearForce> = normWireObject<ChangePlatedShearForce>({ id: normWireRequired(normWireString), newShearForce: normWireRequired(normWireNumber) });
