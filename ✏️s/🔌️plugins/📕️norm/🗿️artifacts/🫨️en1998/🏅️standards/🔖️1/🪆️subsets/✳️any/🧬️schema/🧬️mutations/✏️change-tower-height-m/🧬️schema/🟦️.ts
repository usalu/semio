/** ✏️ `change-tower-height-m` wire twin: the leaf payload `ChangeTowerHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerHeightM {
  mutation: "changeTowerHeightM";
  id: string;
  newHeightM: number;
}

export const parseChangeTowerHeightM: NormWireReader<ChangeTowerHeightM> = normWireObject<ChangeTowerHeightM>({ mutation: normWireRequired(normWireLiteral("changeTowerHeightM")), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
