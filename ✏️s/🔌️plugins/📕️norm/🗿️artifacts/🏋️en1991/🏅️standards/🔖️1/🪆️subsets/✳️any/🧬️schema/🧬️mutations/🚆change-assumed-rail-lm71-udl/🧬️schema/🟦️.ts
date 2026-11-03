/** 🚆 `change-assumed-rail-lm71-udl` wire twin: the leaf payload `ChangeAssumedRailLm71Udl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedRailLm71Udl {
  newAssumedRailLm71Udl: number;
}

export const parseChangeAssumedRailLm71Udl: NormWireReader<ChangeAssumedRailLm71Udl> = normWireObject<ChangeAssumedRailLm71Udl>({ newAssumedRailLm71Udl: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
