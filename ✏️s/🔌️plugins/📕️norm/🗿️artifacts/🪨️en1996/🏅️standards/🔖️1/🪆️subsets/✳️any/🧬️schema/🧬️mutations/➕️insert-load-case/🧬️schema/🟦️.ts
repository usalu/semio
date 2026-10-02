/** ➕️ `insert-load-case` wire twin: the leaf payload `InsertLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseWallLoadCase, type WallLoadCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLoadCase {
  wallIndex: number;
  index: number;
  loadCase: WallLoadCase;
}

export const parseInsertLoadCase: NormWireReader<InsertLoadCase> = normWireObject<InsertLoadCase>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCase: normWireRequired(parseWallLoadCase) });
