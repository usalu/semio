/** 🛢️ `change-silo-shell-shear-stress` wire twin: the leaf payload `ChangeSiloShellShearStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloShellShearStress {
  id: string;
  newShellShearStress: number;
}

export const parseChangeSiloShellShearStress: NormWireReader<ChangeSiloShellShearStress> = normWireObject<ChangeSiloShellShearStress>({ id: normWireRequired(normWireString), newShellShearStress: normWireRequired(normWireNumber) });
