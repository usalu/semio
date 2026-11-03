/** 🔺️ `change-truss-height` wire twin: the leaf payload `ChangeTrussHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussHeight {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussHeight: NormWireReader<ChangeTrussHeight> = normWireObject<ChangeTrussHeight>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
