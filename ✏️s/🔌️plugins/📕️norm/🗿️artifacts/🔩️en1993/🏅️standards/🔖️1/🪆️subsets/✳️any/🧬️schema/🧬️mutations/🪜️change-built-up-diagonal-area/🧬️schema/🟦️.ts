/** 🪜️ `change-built-up-diagonal-area` wire twin: the leaf payload `ChangeBuiltUpDiagonalArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpDiagonalArea {
  id: string;
  newDiagonalArea: number;
}

export const parseChangeBuiltUpDiagonalArea: NormWireReader<ChangeBuiltUpDiagonalArea> = normWireObject<ChangeBuiltUpDiagonalArea>({ id: normWireRequired(normWireString), newDiagonalArea: normWireRequired(normWireNumber) });
