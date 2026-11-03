/** ➖️ `remove-load-case` wire twin: the leaf payload `RemoveLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLoadCase {
  wallId: string;
  id: string;
}

export const parseRemoveLoadCase: NormWireReader<RemoveLoadCase> = normWireObject<RemoveLoadCase>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
