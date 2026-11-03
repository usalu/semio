/** ✏️ `change-bridge-pier-height-m` wire twin: the leaf payload `ChangeBridgePierHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePierHeightM {
  mutation: "changeBridgePierHeightM";
  id: string;
  newPierHeightM: number;
}

export const parseChangeBridgePierHeightM: NormWireReader<ChangeBridgePierHeightM> = normWireObject<ChangeBridgePierHeightM>({ mutation: normWireRequired(normWireLiteral("changeBridgePierHeightM")), id: normWireRequired(normWireString), newPierHeightM: normWireRequired(normWireNumber) });
