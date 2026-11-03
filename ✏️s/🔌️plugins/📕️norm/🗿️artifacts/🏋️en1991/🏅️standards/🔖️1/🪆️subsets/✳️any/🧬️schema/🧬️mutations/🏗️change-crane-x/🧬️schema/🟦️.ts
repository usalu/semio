/** 🏗️ `change-crane-x` wire twin: the leaf payload `ChangeCraneX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneX {
  newCraneX: number;
}

export const parseChangeCraneX: NormWireReader<ChangeCraneX> = normWireObject<ChangeCraneX>({ newCraneX: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
