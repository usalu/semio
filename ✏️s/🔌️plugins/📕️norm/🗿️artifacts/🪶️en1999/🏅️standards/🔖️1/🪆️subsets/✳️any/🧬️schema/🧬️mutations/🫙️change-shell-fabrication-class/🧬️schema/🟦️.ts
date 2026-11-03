/** 🫙 `change-shell-fabrication-class` wire twin: the leaf payload `ChangeShellFabricationClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FabricationClass, parseFabricationClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeShellFabricationClass {
  mutation: "changeShellFabricationClass";
  shellId: string;
  newFabricationClass: FabricationClass;
}

export const parseChangeShellFabricationClass: NormWireReader<ChangeShellFabricationClass> = normWireObject<ChangeShellFabricationClass>({ mutation: normWireRequired(normWireLiteral("changeShellFabricationClass")), shellId: normWireRequired(normWireString), newFabricationClass: normWireRequired(parseFabricationClass) });
