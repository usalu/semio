/** 🚮 `remove-element` wire twin: the leaf payload `RemoveElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface RemoveElement {
  mutation: "removeelement";
  id: string;
}

export const parseRemoveElement: NormWireReader<RemoveElement> = normWireObject<RemoveElement>({ mutation: normWireRequired(normWireLiteral("removeelement")), id: normWireRequired(normWireString) });
