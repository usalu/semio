/** 🛢️ `change-silo-internal-pressure` wire twin: the leaf payload `ChangeSiloInternalPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloInternalPressure {
  id: string;
  newInternalPressure: number;
}

export const parseChangeSiloInternalPressure: NormWireReader<ChangeSiloInternalPressure> = normWireObject<ChangeSiloInternalPressure>({ id: normWireRequired(normWireString), newInternalPressure: normWireRequired(normWireNumber) });
