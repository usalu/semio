/** 🚆 `change-assumed-rail-lm71` wire twin: the leaf payload `ChangeAssumedRailLm71`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedRailLm71 {
  newAssumedRailLm71: number;
}

export const parseChangeAssumedRailLm71: NormWireReader<ChangeAssumedRailLm71> = normWireObject<ChangeAssumedRailLm71>({ newAssumedRailLm71: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
