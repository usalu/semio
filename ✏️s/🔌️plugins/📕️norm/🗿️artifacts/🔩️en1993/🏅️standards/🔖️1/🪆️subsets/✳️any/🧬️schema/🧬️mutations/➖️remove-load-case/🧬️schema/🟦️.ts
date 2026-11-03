/** ➖️ `remove-load-case` wire twin: the leaf payload `RemoveLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLoadCase {
  index: number;
}

export const parseRemoveLoadCase: NormWireReader<RemoveLoadCase> = normWireObject<RemoveLoadCase>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
