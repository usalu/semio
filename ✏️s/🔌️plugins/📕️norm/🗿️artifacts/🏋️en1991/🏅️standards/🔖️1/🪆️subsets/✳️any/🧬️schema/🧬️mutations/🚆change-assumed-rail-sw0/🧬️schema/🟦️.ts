/** 🚆 `change-assumed-rail-sw0` wire twin: the leaf payload `ChangeAssumedRailSw0`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedRailSw0 {
  newAssumedRailSw0: number;
}

export const parseChangeAssumedRailSw0: NormWireReader<ChangeAssumedRailSw0> = normWireObject<ChangeAssumedRailSw0>({ newAssumedRailSw0: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
