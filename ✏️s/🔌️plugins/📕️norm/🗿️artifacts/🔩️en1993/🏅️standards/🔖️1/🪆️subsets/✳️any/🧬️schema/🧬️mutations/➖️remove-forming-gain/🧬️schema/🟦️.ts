/** ➖️ `remove-forming-gain` wire twin: the leaf payload `RemoveFormingGain`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFormingGain {
  index: number;
}

export const parseRemoveFormingGain: NormWireReader<RemoveFormingGain> = normWireObject<RemoveFormingGain>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
