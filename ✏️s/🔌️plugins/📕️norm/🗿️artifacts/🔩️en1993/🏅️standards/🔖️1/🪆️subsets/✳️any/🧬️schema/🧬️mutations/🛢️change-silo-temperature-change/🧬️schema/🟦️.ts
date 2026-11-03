/** 🛢️ `change-silo-temperature-change` wire twin: the leaf payload `ChangeSiloTemperatureChange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloTemperatureChange {
  id: string;
  newTemperatureChange: number;
}

export const parseChangeSiloTemperatureChange: NormWireReader<ChangeSiloTemperatureChange> = normWireObject<ChangeSiloTemperatureChange>({ id: normWireRequired(normWireString), newTemperatureChange: normWireRequired(normWireNumber) });
