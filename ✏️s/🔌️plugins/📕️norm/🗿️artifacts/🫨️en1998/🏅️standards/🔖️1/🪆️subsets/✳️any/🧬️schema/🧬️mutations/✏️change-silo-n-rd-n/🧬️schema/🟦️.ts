/** ✏️ `change-silo-n-rd-n` wire twin: the leaf payload `ChangeSiloNRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloNRdN {
  mutation: "changeSiloNRdN";
  id: string;
  newNRdN: number;
}

export const parseChangeSiloNRdN: NormWireReader<ChangeSiloNRdN> = normWireObject<ChangeSiloNRdN>({ mutation: normWireRequired(normWireLiteral("changeSiloNRdN")), id: normWireRequired(normWireString), newNRdN: normWireRequired(normWireNumber) });
