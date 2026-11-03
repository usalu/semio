/** 🏗️ `change-crane-torsion-constant` wire twin: the leaf payload `ChangeCraneTorsionConstant`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneTorsionConstant {
  id: string;
  newTorsionConstant: number;
}

export const parseChangeCraneTorsionConstant: NormWireReader<ChangeCraneTorsionConstant> = normWireObject<ChangeCraneTorsionConstant>({ id: normWireRequired(normWireString), newTorsionConstant: normWireRequired(normWireNumber) });
