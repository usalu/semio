/** 🏗️ `change-member-vibration-frequency` wire twin: the leaf payload `ChangeMemberVibrationFrequency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberVibrationFrequency {
  mutation: "changeMemberVibrationFrequency";
  id: string;
  newVibrationFrequency: number;
}

export const parseChangeMemberVibrationFrequency: NormWireReader<ChangeMemberVibrationFrequency> = normWireObject<ChangeMemberVibrationFrequency>({ mutation: normWireRequired(normWireLiteral("changeMemberVibrationFrequency")), id: normWireRequired(normWireString), newVibrationFrequency: normWireRequired(normWireNumber) });
