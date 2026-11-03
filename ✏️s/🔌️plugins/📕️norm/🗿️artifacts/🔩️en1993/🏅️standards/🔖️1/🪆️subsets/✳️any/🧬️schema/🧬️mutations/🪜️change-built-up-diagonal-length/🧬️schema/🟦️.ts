/** 🪜️ `change-built-up-diagonal-length` wire twin: the leaf payload `ChangeBuiltUpDiagonalLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpDiagonalLength {
  id: string;
  newDiagonalLength: number;
}

export const parseChangeBuiltUpDiagonalLength: NormWireReader<ChangeBuiltUpDiagonalLength> = normWireObject<ChangeBuiltUpDiagonalLength>({ id: normWireRequired(normWireString), newDiagonalLength: normWireRequired(normWireNumber) });
