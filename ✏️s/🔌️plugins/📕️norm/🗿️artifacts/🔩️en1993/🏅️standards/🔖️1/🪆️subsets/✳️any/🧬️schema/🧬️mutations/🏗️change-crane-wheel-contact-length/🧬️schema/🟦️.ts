/** 🏗️ `change-crane-wheel-contact-length` wire twin: the leaf payload `ChangeCraneWheelContactLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneWheelContactLength {
  id: string;
  newWheelContactLength: number;
}

export const parseChangeCraneWheelContactLength: NormWireReader<ChangeCraneWheelContactLength> = normWireObject<ChangeCraneWheelContactLength>({ id: normWireRequired(normWireString), newWheelContactLength: normWireRequired(normWireNumber) });
