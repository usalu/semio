/** 🔗 `change-tie-run-dead-load` wire twin: the leaf payload `ChangeTieRunDeadLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunDeadLoad {
  id: string;
  newDeadLoad: number;
}

export const parseChangeTieRunDeadLoad: NormWireReader<ChangeTieRunDeadLoad> = normWireObject<ChangeTieRunDeadLoad>({ id: normWireRequired(normWireString), newDeadLoad: normWireRequired(normWireNumber) });
