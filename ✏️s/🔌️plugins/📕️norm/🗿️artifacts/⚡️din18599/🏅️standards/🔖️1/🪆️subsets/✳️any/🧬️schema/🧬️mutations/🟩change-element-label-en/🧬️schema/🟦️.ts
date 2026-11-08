/** 🟩 `change-element-label-en` wire twin: the leaf payload `ChangeElementLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementLabelEn {
  mutation: "changeelementlabelen";
  elementId: string;
  newLabelEn: string;
}

export const parseChangeElementLabelEn: NormWireReader<ChangeElementLabelEn> = normWireObject<ChangeElementLabelEn>({ mutation: normWireRequired(normWireLiteral("changeelementlabelen")), elementId: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
