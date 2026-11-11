/** 🆕 `insert-element` wire twin: the leaf payload `InsertElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type EnvelopeElement, parseEnvelopeElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertElement {
  mutation: "insertelement";
  index: number;
  element: EnvelopeElement;
}

export const parseInsertElement: NormWireReader<InsertElement> = normWireObject<InsertElement>({ mutation: normWireRequired(normWireLiteral("insertelement")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), element: normWireRequired(parseEnvelopeElement) });
