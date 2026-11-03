/** 🧵️ `change-weld-shear-force` wire twin: the leaf payload `ChangeWeldShearForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldShearForce {
  id: string;
  newShearForce: number;
}

export const parseChangeWeldShearForce: NormWireReader<ChangeWeldShearForce> = normWireObject<ChangeWeldShearForce>({ id: normWireRequired(normWireString), newShearForce: normWireRequired(normWireNumber) });
