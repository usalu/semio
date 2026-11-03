/** 🕰️ `change-initial-temperature` wire twin: the leaf payload `ChangeInitialTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeInitialTemperature {
  newT0: number;
}

export const parseChangeInitialTemperature: NormWireReader<ChangeInitialTemperature> = normWireObject<ChangeInitialTemperature>({ newT0: normWireRequired(normWireNumber) });
