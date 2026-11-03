/** 🏗 `change-crane-hoist-rated-load` wire twin: the leaf payload `ChangeCraneHoistRatedLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneHoistRatedLoad {
  id: string;
  newRatedLoad: number;
}

export const parseChangeCraneHoistRatedLoad: NormWireReader<ChangeCraneHoistRatedLoad> = normWireObject<ChangeCraneHoistRatedLoad>({ id: normWireRequired(normWireString), newRatedLoad: normWireRequired(normWireNumber) });
