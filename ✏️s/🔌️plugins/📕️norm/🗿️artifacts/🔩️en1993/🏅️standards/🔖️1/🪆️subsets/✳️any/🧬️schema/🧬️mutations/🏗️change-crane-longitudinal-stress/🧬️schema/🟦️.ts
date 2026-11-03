/** 🏗️ `change-crane-longitudinal-stress` wire twin: the leaf payload `ChangeCraneLongitudinalStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneLongitudinalStress {
  id: string;
  newLongitudinalStress: number;
}

export const parseChangeCraneLongitudinalStress: NormWireReader<ChangeCraneLongitudinalStress> = normWireObject<ChangeCraneLongitudinalStress>({ id: normWireRequired(normWireString), newLongitudinalStress: normWireRequired(normWireNumber) });
