/** ➕️ `insert-self-weight-elements` wire twin: the leaf payload `InsertSelfWeightElements`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSelfWeightElement, type SelfWeightElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSelfWeightElements {
  index: number;
  item: SelfWeightElement;
}

export const parseInsertSelfWeightElements: NormWireReader<InsertSelfWeightElements> = normWireObject<InsertSelfWeightElements>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseSelfWeightElement) });
