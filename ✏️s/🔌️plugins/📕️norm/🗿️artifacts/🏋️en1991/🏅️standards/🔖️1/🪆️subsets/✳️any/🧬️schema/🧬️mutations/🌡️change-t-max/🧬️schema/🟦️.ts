/** 🌡️ `change-t-max` wire twin: the leaf payload `ChangeTMax`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTMax {
  newTMax: number;
}

export const parseChangeTMax: NormWireReader<ChangeTMax> = normWireObject<ChangeTMax>({ newTMax: normWireRequired(normWireNumber) });
