/** 🧱️ `change-element-weld-t-junction` wire twin: the leaf payload `ChangeElementWeldTJunction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementWeldTJunction {
  mutation: "changeElementWeldTJunction";
  sectionId: string;
  elementId: string;
  newWeldTJunction: boolean;
}

export const parseChangeElementWeldTJunction: NormWireReader<ChangeElementWeldTJunction> = normWireObject<ChangeElementWeldTJunction>({ mutation: normWireRequired(normWireLiteral("changeElementWeldTJunction")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWeldTJunction: normWireRequired(normWireBoolean) });
