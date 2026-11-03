/** ✏️ `change-silo-v-rd-n` wire twin: the leaf payload `ChangeSiloVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloVRdN {
  mutation: "changeSiloVRdN";
  id: string;
  newVRdN: number;
}

export const parseChangeSiloVRdN: NormWireReader<ChangeSiloVRdN> = normWireObject<ChangeSiloVRdN>({ mutation: normWireRequired(normWireLiteral("changeSiloVRdN")), id: normWireRequired(normWireString), newVRdN: normWireRequired(normWireNumber) });
