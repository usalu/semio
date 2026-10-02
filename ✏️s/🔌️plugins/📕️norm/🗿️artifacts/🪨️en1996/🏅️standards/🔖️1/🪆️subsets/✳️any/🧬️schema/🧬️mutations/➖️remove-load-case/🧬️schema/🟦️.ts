/** ➖️ `remove-load-case` wire twin: the leaf payload `RemoveLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveLoadCase {
  wallIndex: number;
  index: number;
}

export const parseRemoveLoadCase: NormWireReader<RemoveLoadCase> = normWireObject<RemoveLoadCase>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
