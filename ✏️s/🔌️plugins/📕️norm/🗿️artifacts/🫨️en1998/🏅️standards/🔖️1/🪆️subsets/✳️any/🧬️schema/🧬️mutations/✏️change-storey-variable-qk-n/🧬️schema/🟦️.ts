/** ✏️ `change-storey-variable-qk-n` wire twin: the leaf payload `ChangeStoreyVariableQkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyVariableQkN {
  mutation: "changeStoreyVariableQkN";
  buildingId: string;
  storeyId: string;
  id: string;
  newQkN: number;
}

export const parseChangeStoreyVariableQkN: NormWireReader<ChangeStoreyVariableQkN> = normWireObject<ChangeStoreyVariableQkN>({ mutation: normWireRequired(normWireLiteral("changeStoreyVariableQkN")), buildingId: normWireRequired(normWireString), storeyId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQkN: normWireRequired(normWireNumber) });
