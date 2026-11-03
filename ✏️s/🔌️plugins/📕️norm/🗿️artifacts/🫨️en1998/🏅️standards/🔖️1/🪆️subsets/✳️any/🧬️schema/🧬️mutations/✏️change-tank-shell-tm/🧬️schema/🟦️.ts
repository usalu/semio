/** ✏️ `change-tank-shell-tm` wire twin: the leaf payload `ChangeTankShellTM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankShellTM {
  mutation: "changeTankShellTM";
  id: string;
  newShellThicknessM: number;
}

export const parseChangeTankShellTM: NormWireReader<ChangeTankShellTM> = normWireObject<ChangeTankShellTM>({ mutation: normWireRequired(normWireLiteral("changeTankShellTM")), id: normWireRequired(normWireString), newShellThicknessM: normWireRequired(normWireNumber) });
