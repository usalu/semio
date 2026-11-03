/** ✏️ `change-pier-shear-rd-n` wire twin: the leaf payload `ChangePierShearRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierShearRdN {
  mutation: "changePierShearRdN";
  id: string;
  newShearRdN: number;
}

export const parseChangePierShearRdN: NormWireReader<ChangePierShearRdN> = normWireObject<ChangePierShearRdN>({ mutation: normWireRequired(normWireLiteral("changePierShearRdN")), id: normWireRequired(normWireString), newShearRdN: normWireRequired(normWireNumber) });
