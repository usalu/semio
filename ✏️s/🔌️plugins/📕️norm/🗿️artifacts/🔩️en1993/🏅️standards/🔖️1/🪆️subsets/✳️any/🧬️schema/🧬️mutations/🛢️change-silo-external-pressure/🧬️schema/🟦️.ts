/** 🛢️ `change-silo-external-pressure` wire twin: the leaf payload `ChangeSiloExternalPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloExternalPressure {
  id: string;
  newExternalPressure: number;
}

export const parseChangeSiloExternalPressure: NormWireReader<ChangeSiloExternalPressure> = normWireObject<ChangeSiloExternalPressure>({ id: normWireRequired(normWireString), newExternalPressure: normWireRequired(normWireNumber) });
