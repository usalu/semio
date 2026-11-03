/** ⚙️ `change-silo-k` wire twin: the leaf payload `ChangeSiloK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloK {
  newSiloK: number;
}

export const parseChangeSiloK: NormWireReader<ChangeSiloK> = normWireObject<ChangeSiloK>({ newSiloK: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
