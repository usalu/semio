/** 🔎 `change-silo-mu` wire twin: the leaf payload `ChangeSiloMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloMu {
  newSiloMu: number;
}

export const parseChangeSiloMu: NormWireReader<ChangeSiloMu> = normWireObject<ChangeSiloMu>({ newSiloMu: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
