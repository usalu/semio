/** 🟩 `change-element-label-en` wire twin: the leaf payload `ChangeElementLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementLabelEn {
  mutation: "changeelementlabelen";
  elementId: string;
  newLabelEn: string;
}

export const parseChangeElementLabelEn: NormWireReader<ChangeElementLabelEn> = normWireObject<ChangeElementLabelEn>({ mutation: normWireRequired(normWireLiteral("changeelementlabelen")), elementId: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
