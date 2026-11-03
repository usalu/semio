/** 🔗 `change-tie-run-imposed-load` wire twin: the leaf payload `ChangeTieRunImposedLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunImposedLoad {
  id: string;
  newImposedLoad: number;
}

export const parseChangeTieRunImposedLoad: NormWireReader<ChangeTieRunImposedLoad> = normWireObject<ChangeTieRunImposedLoad>({ id: normWireRequired(normWireString), newImposedLoad: normWireRequired(normWireNumber) });
