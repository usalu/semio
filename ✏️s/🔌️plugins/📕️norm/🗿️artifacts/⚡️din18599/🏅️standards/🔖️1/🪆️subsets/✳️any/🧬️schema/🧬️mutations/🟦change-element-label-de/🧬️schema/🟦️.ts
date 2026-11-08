/** 🟦 `change-element-label-de` wire twin: the leaf payload `ChangeElementLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementLabelDe {
  mutation: "changeelementlabelde";
  elementId: string;
  newLabelDe: string;
}

export const parseChangeElementLabelDe: NormWireReader<ChangeElementLabelDe> = normWireObject<ChangeElementLabelDe>({ mutation: normWireRequired(normWireLiteral("changeelementlabelde")), elementId: normWireRequired(normWireString), newLabelDe: normWireRequired(normWireString) });
