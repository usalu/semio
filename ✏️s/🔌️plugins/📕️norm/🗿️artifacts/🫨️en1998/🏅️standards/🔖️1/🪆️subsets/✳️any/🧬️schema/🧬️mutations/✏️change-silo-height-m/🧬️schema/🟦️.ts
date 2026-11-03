/** ✏️ `change-silo-height-m` wire twin: the leaf payload `ChangeSiloHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloHeightM {
  mutation: "changeSiloHeightM";
  id: string;
  newHeightM: number;
}

export const parseChangeSiloHeightM: NormWireReader<ChangeSiloHeightM> = normWireObject<ChangeSiloHeightM>({ mutation: normWireRequired(normWireLiteral("changeSiloHeightM")), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
