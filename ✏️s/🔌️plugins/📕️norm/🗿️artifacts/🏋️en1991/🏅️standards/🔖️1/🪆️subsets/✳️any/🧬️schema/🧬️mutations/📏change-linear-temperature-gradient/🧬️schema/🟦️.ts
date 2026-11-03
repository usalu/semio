/** 📏 `change-linear-temperature-gradient` wire twin: the leaf payload `ChangeLinearTemperatureGradient`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLinearTemperatureGradient {
  newDeltaTM: number;
}

export const parseChangeLinearTemperatureGradient: NormWireReader<ChangeLinearTemperatureGradient> = normWireObject<ChangeLinearTemperatureGradient>({ newDeltaTM: normWireRequired(normWireNumber) });
