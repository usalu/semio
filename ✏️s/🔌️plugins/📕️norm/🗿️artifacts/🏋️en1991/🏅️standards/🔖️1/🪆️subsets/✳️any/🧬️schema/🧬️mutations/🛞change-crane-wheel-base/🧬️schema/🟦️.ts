/** 🛞 `change-crane-wheel-base` wire twin: the leaf payload `ChangeCraneWheelBase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneWheelBase {
  newCraneWheelBase: number;
}

export const parseChangeCraneWheelBase: NormWireReader<ChangeCraneWheelBase> = normWireObject<ChangeCraneWheelBase>({ newCraneWheelBase: normWireRequired(normWireRange(normWireNumber, {"minimum":0.5})) });
