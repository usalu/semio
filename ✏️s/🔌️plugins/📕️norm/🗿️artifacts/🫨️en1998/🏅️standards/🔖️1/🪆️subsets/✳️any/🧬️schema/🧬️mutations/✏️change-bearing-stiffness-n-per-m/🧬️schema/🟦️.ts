/** ✏️ `change-bearing-stiffness-n-per-m` wire twin: the leaf payload `ChangeBearingStiffnessNPerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingStiffnessNPerM {
  mutation: "changeBearingStiffnessNPerM";
  id: string;
  newStiffnessNPerM: number;
}

export const parseChangeBearingStiffnessNPerM: NormWireReader<ChangeBearingStiffnessNPerM> = normWireObject<ChangeBearingStiffnessNPerM>({ mutation: normWireRequired(normWireLiteral("changeBearingStiffnessNPerM")), id: normWireRequired(normWireString), newStiffnessNPerM: normWireRequired(normWireNumber) });
