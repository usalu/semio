/** 🛢️ `change-silo-depth` wire twin: the leaf payload `ChangeSiloDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloDepth {
  id: string;
  newDepth: number;
}

export const parseChangeSiloDepth: NormWireReader<ChangeSiloDepth> = normWireObject<ChangeSiloDepth>({ id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
