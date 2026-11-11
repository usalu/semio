/** 🏠️ `insert-element` wire twin: the leaf payload `InsertElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type Din4108EnvelopeElement, parseDin4108EnvelopeElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertElement {
  index: number | null;
  element: Din4108EnvelopeElement;
}

export const parseInsertElement: NormWireReader<InsertElement> = normWireObject<InsertElement>({ index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), element: normWireRequired(parseDin4108EnvelopeElement) });
