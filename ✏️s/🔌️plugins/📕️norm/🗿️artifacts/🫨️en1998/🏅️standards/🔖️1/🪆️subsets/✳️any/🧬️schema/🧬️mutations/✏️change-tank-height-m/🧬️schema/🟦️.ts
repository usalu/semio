/** ✏️ `change-tank-height-m` wire twin: the leaf payload `ChangeTankHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankHeightM {
  mutation: "changeTankHeightM";
  id: string;
  newHeightM: number;
}

export const parseChangeTankHeightM: NormWireReader<ChangeTankHeightM> = normWireObject<ChangeTankHeightM>({ mutation: normWireRequired(normWireLiteral("changeTankHeightM")), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
