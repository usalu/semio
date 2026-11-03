/** 🌀 `change-assumed-silo-pressure` wire twin: the leaf payload `ChangeAssumedSiloPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedSiloPressure {
  newAssumedSiloPressure: number;
}

export const parseChangeAssumedSiloPressure: NormWireReader<ChangeAssumedSiloPressure> = normWireObject<ChangeAssumedSiloPressure>({ newAssumedSiloPressure: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
