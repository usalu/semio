/** ✏️ `change-tank-v-rd-n` wire twin: the leaf payload `ChangeTankVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankVRdN {
  mutation: "changeTankVRdN";
  id: string;
  newVRdN: number;
}

export const parseChangeTankVRdN: NormWireReader<ChangeTankVRdN> = normWireObject<ChangeTankVRdN>({ mutation: normWireRequired(normWireLiteral("changeTankVRdN")), id: normWireRequired(normWireString), newVRdN: normWireRequired(normWireNumber) });
