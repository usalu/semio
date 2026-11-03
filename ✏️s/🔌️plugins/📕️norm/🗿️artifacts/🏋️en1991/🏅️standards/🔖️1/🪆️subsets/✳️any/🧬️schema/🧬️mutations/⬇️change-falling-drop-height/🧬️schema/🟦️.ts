/** ⬇ `change-falling-drop-height` wire twin: the leaf payload `ChangeFallingDropHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFallingDropHeight {
  id: string;
  newDropHeight: number;
}

export const parseChangeFallingDropHeight: NormWireReader<ChangeFallingDropHeight> = normWireObject<ChangeFallingDropHeight>({ id: normWireRequired(normWireString), newDropHeight: normWireRequired(normWireNumber) });
