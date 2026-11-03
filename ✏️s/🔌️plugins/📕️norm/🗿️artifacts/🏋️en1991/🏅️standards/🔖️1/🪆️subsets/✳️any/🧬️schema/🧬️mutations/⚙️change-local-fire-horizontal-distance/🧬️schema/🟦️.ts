/** ⚙ `change-local-fire-horizontal-distance` wire twin: the leaf payload `ChangeLocalFireHorizontalDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLocalFireHorizontalDistance {
  id: string;
  newHorizontalDistance: number;
}

export const parseChangeLocalFireHorizontalDistance: NormWireReader<ChangeLocalFireHorizontalDistance> = normWireObject<ChangeLocalFireHorizontalDistance>({ id: normWireRequired(normWireString), newHorizontalDistance: normWireRequired(normWireNumber) });
