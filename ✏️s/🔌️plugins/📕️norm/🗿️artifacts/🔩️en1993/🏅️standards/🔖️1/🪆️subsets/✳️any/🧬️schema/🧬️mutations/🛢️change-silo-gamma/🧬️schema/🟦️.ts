/** 🛢️ `change-silo-gamma` wire twin: the leaf payload `ChangeSiloGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloGamma {
  id: string;
  newGamma: number;
}

export const parseChangeSiloGamma: NormWireReader<ChangeSiloGamma> = normWireObject<ChangeSiloGamma>({ id: normWireRequired(normWireString), newGamma: normWireRequired(normWireNumber) });
