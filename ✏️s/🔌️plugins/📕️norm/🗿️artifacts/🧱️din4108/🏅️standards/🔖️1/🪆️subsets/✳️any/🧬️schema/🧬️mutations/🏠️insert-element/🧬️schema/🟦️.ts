/** 🏠️ `insert-element` wire twin: the leaf payload `InsertElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108EnvelopeElement, parseDin4108EnvelopeElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertElement {
  index: number;
  element: Din4108EnvelopeElement;
}

export const parseInsertElement: NormWireReader<InsertElement> = normWireObject<InsertElement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), element: normWireRequired(parseDin4108EnvelopeElement) });
