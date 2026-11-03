/** 🧊 `change-t-min` wire twin: the leaf payload `ChangeTMin`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTMin {
  newTMin: number;
}

export const parseChangeTMin: NormWireReader<ChangeTMin> = normWireObject<ChangeTMin>({ newTMin: normWireRequired(normWireNumber) });
