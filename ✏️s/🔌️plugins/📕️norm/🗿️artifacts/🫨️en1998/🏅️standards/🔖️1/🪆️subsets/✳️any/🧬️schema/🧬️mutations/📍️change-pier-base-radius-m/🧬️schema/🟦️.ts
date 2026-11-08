/** 📍️ `change-pier-base-radius-m` wire twin: the leaf payload `ChangePierBaseRadiusM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBaseRadiusM {
  mutation: "changePierBaseRadiusM";
  id: string;
  newRadiusM: number;
}

export const parseChangePierBaseRadiusM: NormWireReader<ChangePierBaseRadiusM> = normWireObject<ChangePierBaseRadiusM>({ mutation: normWireRequired(normWireLiteral("changePierBaseRadiusM")), id: normWireRequired(normWireString), newRadiusM: normWireRequired(normWireNumber) });
