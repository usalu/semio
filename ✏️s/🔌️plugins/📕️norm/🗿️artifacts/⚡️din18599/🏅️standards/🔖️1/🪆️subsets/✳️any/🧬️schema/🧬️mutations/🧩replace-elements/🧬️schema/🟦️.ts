/** 🧩 `replace-elements` wire twin: the leaf payload `ReplaceElements`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type EnvelopeElement, parseEnvelopeElement } from "../../../📸️snapshot/🟦️.ts";

export interface ReplaceElements {
  mutation: "replaceElements";
  newElements: EnvelopeElement[];
}

export const parseReplaceElements: NormWireReader<ReplaceElements> = normWireObject<ReplaceElements>({ mutation: normWireRequired(normWireLiteral("replaceElements")), newElements: normWireRequired(normWireArray(parseEnvelopeElement)) });
