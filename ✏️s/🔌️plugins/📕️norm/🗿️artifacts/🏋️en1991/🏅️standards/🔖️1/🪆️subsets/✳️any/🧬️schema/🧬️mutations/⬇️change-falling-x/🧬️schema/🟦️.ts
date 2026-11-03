/** ⬇ `change-falling-x` wire twin: the leaf payload `ChangeFallingX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFallingX {
  id: string;
  newX: number;
}

export const parseChangeFallingX: NormWireReader<ChangeFallingX> = normWireObject<ChangeFallingX>({ id: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
