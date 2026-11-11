/** 🟦 `change-element-label-de` wire twin: the leaf payload `ChangeElementLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementLabelDe {
  mutation: "changeelementlabelde";
  elementId: string;
  newLabelDe: string;
}

export const parseChangeElementLabelDe: NormWireReader<ChangeElementLabelDe> = normWireObject<ChangeElementLabelDe>({ mutation: normWireRequired(normWireLiteral("changeelementlabelde")), elementId: normWireRequired(normWireString), newLabelDe: normWireRequired(normWireString) });
