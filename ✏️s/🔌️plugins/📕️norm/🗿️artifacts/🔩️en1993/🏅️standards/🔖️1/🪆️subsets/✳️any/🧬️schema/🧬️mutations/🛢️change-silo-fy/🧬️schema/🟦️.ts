/** 🛢️ `change-silo-fy` wire twin: the leaf payload `ChangeSiloFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloFy {
  id: string;
  newFy: number;
}

export const parseChangeSiloFy: NormWireReader<ChangeSiloFy> = normWireObject<ChangeSiloFy>({ id: normWireRequired(normWireString), newFy: normWireRequired(normWireNumber) });
