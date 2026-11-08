/** 🟪 `change-element-kind` wire twin: the leaf payload `ChangeElementKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599ElementKind, parseDin18599ElementKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementKind {
  mutation: "changeelementkind";
  elementId: string;
  newKind: Din18599ElementKind;
}

export const parseChangeElementKind: NormWireReader<ChangeElementKind> = normWireObject<ChangeElementKind>({ mutation: normWireRequired(normWireLiteral("changeelementkind")), elementId: normWireRequired(normWireString), newKind: normWireRequired(parseDin18599ElementKind) });
