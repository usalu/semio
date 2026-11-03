/** ✏️ `change-tank-shell-str-mpa` wire twin: the leaf payload `ChangeTankShellStrMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankShellStrMpa {
  mutation: "changeTankShellStrMpa";
  id: string;
  newShellStrengthMpa: number;
}

export const parseChangeTankShellStrMpa: NormWireReader<ChangeTankShellStrMpa> = normWireObject<ChangeTankShellStrMpa>({ mutation: normWireRequired(normWireLiteral("changeTankShellStrMpa")), id: normWireRequired(normWireString), newShellStrengthMpa: normWireRequired(normWireNumber) });
