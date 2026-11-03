/** 📦 `change-assumed-silo-patch` wire twin: the leaf payload `ChangeAssumedSiloPatch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedSiloPatch {
  newAssumedSiloPatch: number;
}

export const parseChangeAssumedSiloPatch: NormWireReader<ChangeAssumedSiloPatch> = normWireObject<ChangeAssumedSiloPatch>({ newAssumedSiloPatch: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
