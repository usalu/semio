/** 🚁 `change-helicopter-mass` wire twin: the leaf payload `ChangeHelicopterMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHelicopterMass {
  id: string;
  newMass: number;
}

export const parseChangeHelicopterMass: NormWireReader<ChangeHelicopterMass> = normWireObject<ChangeHelicopterMass>({ id: normWireRequired(normWireString), newMass: normWireRequired(normWireNumber) });
