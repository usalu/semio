/** 🏋️ `change-variable-group` wire twin: the leaf payload `ChangeVariableGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVariableGroup {
  mutation: "changeVariableGroup";
  id: string;
  newGroup: string;
}

export const parseChangeVariableGroup: NormWireReader<ChangeVariableGroup> = normWireObject<ChangeVariableGroup>({ mutation: normWireRequired(normWireLiteral("changeVariableGroup")), id: normWireRequired(normWireString), newGroup: normWireRequired(normWireString) });
