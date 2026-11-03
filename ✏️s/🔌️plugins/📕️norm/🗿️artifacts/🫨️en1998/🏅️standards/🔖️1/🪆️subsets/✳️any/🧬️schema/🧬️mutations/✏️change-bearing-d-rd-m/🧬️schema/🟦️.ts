/** ✏️ `change-bearing-d-rd-m` wire twin: the leaf payload `ChangeBearingDRdM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingDRdM {
  mutation: "changeBearingDRdM";
  id: string;
  newDRdM: number;
}

export const parseChangeBearingDRdM: NormWireReader<ChangeBearingDRdM> = normWireObject<ChangeBearingDRdM>({ mutation: normWireRequired(normWireLiteral("changeBearingDRdM")), id: normWireRequired(normWireString), newDRdM: normWireRequired(normWireNumber) });
