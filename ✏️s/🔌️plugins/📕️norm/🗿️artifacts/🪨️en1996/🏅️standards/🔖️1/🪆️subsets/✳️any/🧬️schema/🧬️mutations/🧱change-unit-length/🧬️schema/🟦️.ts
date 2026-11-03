/** 🧱 `change-unit-length` wire twin: the leaf payload `ChangeUnitLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUnitLength {
  id: string;
  newUnitLengthM: number;
}

export const parseChangeUnitLength: NormWireReader<ChangeUnitLength> = normWireObject<ChangeUnitLength>({ id: normWireRequired(normWireString), newUnitLengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
