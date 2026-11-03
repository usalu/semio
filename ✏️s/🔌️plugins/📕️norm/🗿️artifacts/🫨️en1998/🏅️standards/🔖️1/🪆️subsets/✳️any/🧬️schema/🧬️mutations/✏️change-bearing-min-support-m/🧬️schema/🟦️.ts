/** ✏️ `change-bearing-min-support-m` wire twin: the leaf payload `ChangeBearingMinSupportM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingMinSupportM {
  mutation: "changeBearingMinSupportM";
  id: string;
  newMinSupportLengthM: number;
}

export const parseChangeBearingMinSupportM: NormWireReader<ChangeBearingMinSupportM> = normWireObject<ChangeBearingMinSupportM>({ mutation: normWireRequired(normWireLiteral("changeBearingMinSupportM")), id: normWireRequired(normWireString), newMinSupportLengthM: normWireRequired(normWireNumber) });
