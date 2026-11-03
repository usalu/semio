/** 🚗 `change-accidental-explosion-mass` wire twin: the leaf payload `ChangeAccidentalExplosionMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalExplosionMass {
  id: string;
  newExplosionMass: number;
}

export const parseChangeAccidentalExplosionMass: NormWireReader<ChangeAccidentalExplosionMass> = normWireObject<ChangeAccidentalExplosionMass>({ id: normWireRequired(normWireString), newExplosionMass: normWireRequired(normWireNumber) });
