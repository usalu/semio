/** ✏️ `change-bridge-bearing-d-rd-m` wire twin: the leaf payload `ChangeBridgeBearingDRdM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeBearingDRdM {
  mutation: "changeBridgeBearingDRdM";
  id: string;
  newBearingDRdM: number;
}

export const parseChangeBridgeBearingDRdM: NormWireReader<ChangeBridgeBearingDRdM> = normWireObject<ChangeBridgeBearingDRdM>({ mutation: normWireRequired(normWireLiteral("changeBridgeBearingDRdM")), id: normWireRequired(normWireString), newBearingDRdM: normWireRequired(normWireNumber) });
