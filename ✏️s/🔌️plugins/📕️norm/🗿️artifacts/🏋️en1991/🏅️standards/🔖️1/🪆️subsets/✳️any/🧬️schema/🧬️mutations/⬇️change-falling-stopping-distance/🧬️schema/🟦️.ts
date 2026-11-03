/** ⬇ `change-falling-stopping-distance` wire twin: the leaf payload `ChangeFallingStoppingDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFallingStoppingDistance {
  id: string;
  newStoppingDistance: number;
}

export const parseChangeFallingStoppingDistance: NormWireReader<ChangeFallingStoppingDistance> = normWireObject<ChangeFallingStoppingDistance>({ id: normWireRequired(normWireString), newStoppingDistance: normWireRequired(normWireNumber) });
