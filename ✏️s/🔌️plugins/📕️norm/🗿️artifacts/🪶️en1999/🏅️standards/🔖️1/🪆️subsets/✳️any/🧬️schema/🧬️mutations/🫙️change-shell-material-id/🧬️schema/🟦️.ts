/** 🫙 `change-shell-material-id` wire twin: the leaf payload `ChangeShellMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellMaterialId {
  mutation: "changeShellMaterialId";
  shellId: string;
  newMaterialId: string;
}

export const parseChangeShellMaterialId: NormWireReader<ChangeShellMaterialId> = normWireObject<ChangeShellMaterialId>({ mutation: normWireRequired(normWireLiteral("changeShellMaterialId")), shellId: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
