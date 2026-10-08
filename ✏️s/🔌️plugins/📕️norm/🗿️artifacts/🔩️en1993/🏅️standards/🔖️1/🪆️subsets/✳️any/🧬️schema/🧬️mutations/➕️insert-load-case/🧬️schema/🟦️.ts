/** ➕️ `insert-load-case` wire twin: the leaf payload `InsertLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type LoadCase, parseLoadCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLoadCase {
  index?: number | null;
  loadCase: LoadCase;
}

export const parseInsertLoadCase: NormWireReader<InsertLoadCase> = normWireObject<InsertLoadCase>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), loadCase: normWireRequired(parseLoadCase) });
