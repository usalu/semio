/** 📏️ `change-member-radiation-shift` wire twin: the leaf payload `ChangeMemberRadiationShift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRadiationShift {
  id: string;
  newRadiationShift: number;
}

export const parseChangeMemberRadiationShift: NormWireReader<ChangeMemberRadiationShift> = normWireObject<ChangeMemberRadiationShift>({ id: normWireRequired(normWireString), newRadiationShift: normWireRequired(normWireNumber) });
