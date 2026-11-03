/** 🏗️ `change-crane-damage-exponent` wire twin: the leaf payload `ChangeCraneDamageExponent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneDamageExponent {
  id: string;
  newDamageExponent: number;
}

export const parseChangeCraneDamageExponent: NormWireReader<ChangeCraneDamageExponent> = normWireObject<ChangeCraneDamageExponent>({ id: normWireRequired(normWireString), newDamageExponent: normWireRequired(normWireNumber) });
