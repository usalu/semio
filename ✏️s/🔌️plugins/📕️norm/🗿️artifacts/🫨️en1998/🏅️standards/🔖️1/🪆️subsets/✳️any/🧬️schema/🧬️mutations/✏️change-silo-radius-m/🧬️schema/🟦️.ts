/** ✏️ `change-silo-radius-m` wire twin: the leaf payload `ChangeSiloRadiusM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRadiusM {
  mutation: "changeSiloRadiusM";
  id: string;
  newRadiusM: number;
}

export const parseChangeSiloRadiusM: NormWireReader<ChangeSiloRadiusM> = normWireObject<ChangeSiloRadiusM>({ mutation: normWireRequired(normWireLiteral("changeSiloRadiusM")), id: normWireRequired(normWireString), newRadiusM: normWireRequired(normWireNumber) });
