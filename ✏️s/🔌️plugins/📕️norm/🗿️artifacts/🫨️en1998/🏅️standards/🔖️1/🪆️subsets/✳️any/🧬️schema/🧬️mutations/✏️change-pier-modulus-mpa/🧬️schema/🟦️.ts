/** ✏️ `change-pier-modulus-mpa` wire twin: the leaf payload `ChangePierModulusMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierModulusMpa {
  mutation: "changePierModulusMpa";
  id: string;
  newModulusMpa: number;
}

export const parseChangePierModulusMpa: NormWireReader<ChangePierModulusMpa> = normWireObject<ChangePierModulusMpa>({ mutation: normWireRequired(normWireLiteral("changePierModulusMpa")), id: normWireRequired(normWireString), newModulusMpa: normWireRequired(normWireNumber) });
