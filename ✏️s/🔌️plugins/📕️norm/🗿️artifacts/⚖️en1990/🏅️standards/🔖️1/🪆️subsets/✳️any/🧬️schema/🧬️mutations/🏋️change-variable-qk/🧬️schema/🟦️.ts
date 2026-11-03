/** 🏋️ `change-variable-qk` wire twin: the leaf payload `ChangeVariableQk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVariableQk {
  mutation: "changeVariableQk";
  id: string;
  newQk: number;
}

export const parseChangeVariableQk: NormWireReader<ChangeVariableQk> = normWireObject<ChangeVariableQk>({ mutation: normWireRequired(normWireLiteral("changeVariableQk")), id: normWireRequired(normWireString), newQk: normWireRequired(normWireNumber) });
