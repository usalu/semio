/** 🔺️ `change-truss-restraint-length` wire twin: the leaf payload `ChangeTrussRestraintLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussRestraintLength {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussRestraintLength: NormWireReader<ChangeTrussRestraintLength> = normWireObject<ChangeTrussRestraintLength>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
