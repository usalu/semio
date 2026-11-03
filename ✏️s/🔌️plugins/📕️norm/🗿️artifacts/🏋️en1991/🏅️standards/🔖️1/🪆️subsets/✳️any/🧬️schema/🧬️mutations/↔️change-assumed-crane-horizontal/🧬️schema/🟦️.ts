/** ↔️ `change-assumed-crane-horizontal` wire twin: the leaf payload `ChangeAssumedCraneHorizontal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedCraneHorizontal {
  newAssumedCraneHorizontal: number;
}

export const parseChangeAssumedCraneHorizontal: NormWireReader<ChangeAssumedCraneHorizontal> = normWireObject<ChangeAssumedCraneHorizontal>({ newAssumedCraneHorizontal: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
