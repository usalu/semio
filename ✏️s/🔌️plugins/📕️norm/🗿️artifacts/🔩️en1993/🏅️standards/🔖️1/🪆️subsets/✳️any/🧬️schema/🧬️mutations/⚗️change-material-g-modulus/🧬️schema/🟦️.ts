/** ⚗️ `change-material-g-modulus` wire twin: the leaf payload `ChangeMaterialGModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialGModulus {
  id: string;
  newGModulus: number;
}

export const parseChangeMaterialGModulus: NormWireReader<ChangeMaterialGModulus> = normWireObject<ChangeMaterialGModulus>({ id: normWireRequired(normWireString), newGModulus: normWireRequired(normWireNumber) });
