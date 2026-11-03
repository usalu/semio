/** ➖ `change-assumed-crane-wheel` wire twin: the leaf payload `ChangeAssumedCraneWheel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedCraneWheel {
  newAssumedCraneWheel: number;
}

export const parseChangeAssumedCraneWheel: NormWireReader<ChangeAssumedCraneWheel> = normWireObject<ChangeAssumedCraneWheel>({ newAssumedCraneWheel: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
