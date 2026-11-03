/** 📏️ `change-delta-c-dev` wire twin: the leaf payload `ChangeDeltaCDev`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeltaCDev {
  newDeltaCDev: number;
}

export const parseChangeDeltaCDev: NormWireReader<ChangeDeltaCDev> = normWireObject<ChangeDeltaCDev>({ newDeltaCDev: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
