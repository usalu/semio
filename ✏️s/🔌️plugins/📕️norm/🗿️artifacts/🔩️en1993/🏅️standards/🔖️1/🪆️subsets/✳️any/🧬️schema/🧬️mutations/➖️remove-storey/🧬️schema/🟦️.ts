/** ➖️ `remove-storey` wire twin: the leaf payload `RemoveStorey`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveStorey {
  index: number;
}

export const parseRemoveStorey: NormWireReader<RemoveStorey> = normWireObject<RemoveStorey>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
