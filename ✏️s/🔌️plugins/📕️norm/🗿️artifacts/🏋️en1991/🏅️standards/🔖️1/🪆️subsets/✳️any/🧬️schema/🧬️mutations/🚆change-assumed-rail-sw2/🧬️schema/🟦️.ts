/** 🚆 `change-assumed-rail-sw2` wire twin: the leaf payload `ChangeAssumedRailSw2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedRailSw2 {
  newAssumedRailSw2: number;
}

export const parseChangeAssumedRailSw2: NormWireReader<ChangeAssumedRailSw2> = normWireObject<ChangeAssumedRailSw2>({ newAssumedRailSw2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
