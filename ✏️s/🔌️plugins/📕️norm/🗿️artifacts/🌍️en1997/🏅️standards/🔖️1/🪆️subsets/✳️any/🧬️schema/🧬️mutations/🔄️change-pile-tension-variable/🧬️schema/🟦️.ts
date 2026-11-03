/** 🔄️ `change-pile-tension-variable` wire twin: the leaf payload `ChangePileTensionVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileTensionVariable {
  mutation: "changePileTensionVariable";
  id: string;
  newTensionVariable: number;
}

export const parseChangePileTensionVariable: NormWireReader<ChangePileTensionVariable> = normWireObject<ChangePileTensionVariable>({ mutation: normWireRequired(normWireLiteral("changePileTensionVariable")), id: normWireRequired(normWireString), newTensionVariable: normWireRequired(normWireNumber) });
