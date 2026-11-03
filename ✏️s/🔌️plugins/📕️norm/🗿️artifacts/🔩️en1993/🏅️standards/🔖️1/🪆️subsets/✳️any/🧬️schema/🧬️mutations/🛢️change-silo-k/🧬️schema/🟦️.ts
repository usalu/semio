/** 🛢️ `change-silo-k` wire twin: the leaf payload `ChangeSiloK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloK {
  id: string;
  newK: number;
}

export const parseChangeSiloK: NormWireReader<ChangeSiloK> = normWireObject<ChangeSiloK>({ id: normWireRequired(normWireString), newK: normWireRequired(normWireNumber) });
