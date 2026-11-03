/** ✏️ `change-tower-variable-qk-n` wire twin: the leaf payload `ChangeTowerVariableQkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerVariableQkN {
  mutation: "changeTowerVariableQkN";
  towerId: string;
  id: string;
  newQkN: number;
}

export const parseChangeTowerVariableQkN: NormWireReader<ChangeTowerVariableQkN> = normWireObject<ChangeTowerVariableQkN>({ mutation: normWireRequired(normWireLiteral("changeTowerVariableQkN")), towerId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQkN: normWireRequired(normWireNumber) });
