/** 🧱️ `change-element-weld-direction` wire twin: the leaf payload `ChangeElementWeldDirection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldDirection, parseWeldDirection } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementWeldDirection {
  mutation: "changeElementWeldDirection";
  sectionId: string;
  elementId: string;
  newWeldDirection: WeldDirection;
}

export const parseChangeElementWeldDirection: NormWireReader<ChangeElementWeldDirection> = normWireObject<ChangeElementWeldDirection>({ mutation: normWireRequired(normWireLiteral("changeElementWeldDirection")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWeldDirection: normWireRequired(parseWeldDirection) });
