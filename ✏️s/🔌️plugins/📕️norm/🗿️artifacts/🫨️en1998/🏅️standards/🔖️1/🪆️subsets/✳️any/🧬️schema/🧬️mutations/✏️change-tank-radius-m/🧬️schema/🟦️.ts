/** ✏️ `change-tank-radius-m` wire twin: the leaf payload `ChangeTankRadiusM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankRadiusM {
  mutation: "changeTankRadiusM";
  id: string;
  newRadiusM: number;
}

export const parseChangeTankRadiusM: NormWireReader<ChangeTankRadiusM> = normWireObject<ChangeTankRadiusM>({ mutation: normWireRequired(normWireLiteral("changeTankRadiusM")), id: normWireRequired(normWireString), newRadiusM: normWireRequired(normWireNumber) });
