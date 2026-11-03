/** ✏️ `change-pier-steel-fyk-mpa` wire twin: the leaf payload `ChangePierSteelFykMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierSteelFykMpa {
  mutation: "changePierSteelFykMpa";
  id: string;
  newSteelFykMpa: number;
}

export const parseChangePierSteelFykMpa: NormWireReader<ChangePierSteelFykMpa> = normWireObject<ChangePierSteelFykMpa>({ mutation: normWireRequired(normWireLiteral("changePierSteelFykMpa")), id: normWireRequired(normWireString), newSteelFykMpa: normWireRequired(normWireNumber) });
