/** 🏗️ `change-crane-max-wheel-load` wire twin: the leaf payload `ChangeCraneMaxWheelLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneMaxWheelLoad {
  id: string;
  newMaxWheelLoad: number;
}

export const parseChangeCraneMaxWheelLoad: NormWireReader<ChangeCraneMaxWheelLoad> = normWireObject<ChangeCraneMaxWheelLoad>({ id: normWireRequired(normWireString), newMaxWheelLoad: normWireRequired(normWireNumber) });
