/** 🏗️ `change-member-vib-freq-min` wire twin: the leaf payload `ChangeMemberVibFreqMin`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberVibFreqMin {
  mutation: "changeMemberVibFreqMin";
  id: string;
  newVibrationFrequencyMin: number;
}

export const parseChangeMemberVibFreqMin: NormWireReader<ChangeMemberVibFreqMin> = normWireObject<ChangeMemberVibFreqMin>({ mutation: normWireRequired(normWireLiteral("changeMemberVibFreqMin")), id: normWireRequired(normWireString), newVibrationFrequencyMin: normWireRequired(normWireNumber) });
