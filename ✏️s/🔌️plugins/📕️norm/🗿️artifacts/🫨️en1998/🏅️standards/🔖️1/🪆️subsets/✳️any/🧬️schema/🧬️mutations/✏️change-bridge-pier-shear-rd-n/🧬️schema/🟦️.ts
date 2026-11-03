/** ✏️ `change-bridge-pier-shear-rd-n` wire twin: the leaf payload `ChangeBridgePierShearRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePierShearRdN {
  mutation: "changeBridgePierShearRdN";
  id: string;
  newPierShearRdN: number;
}

export const parseChangeBridgePierShearRdN: NormWireReader<ChangeBridgePierShearRdN> = normWireObject<ChangeBridgePierShearRdN>({ mutation: normWireRequired(normWireLiteral("changeBridgePierShearRdN")), id: normWireRequired(normWireString), newPierShearRdN: normWireRequired(normWireNumber) });
