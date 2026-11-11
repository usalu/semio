/** 🟪 `change-element-kind` wire twin: the leaf payload `ChangeElementKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599ElementKind, parseDin18599ElementKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementKind {
  mutation: "changeelementkind";
  elementId: string;
  newKind: Din18599ElementKind;
}

export const parseChangeElementKind: NormWireReader<ChangeElementKind> = normWireObject<ChangeElementKind>({ mutation: normWireRequired(normWireLiteral("changeelementkind")), elementId: normWireRequired(normWireString), newKind: normWireRequired(parseDin18599ElementKind) });
