/** 📏️ `change-member-stress-shift` wire twin: the leaf payload `ChangeMemberStressShift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStressShift {
  id: string;
  newStressShift: number;
}

export const parseChangeMemberStressShift: NormWireReader<ChangeMemberStressShift> = normWireObject<ChangeMemberStressShift>({ id: normWireRequired(normWireString), newStressShift: normWireRequired(normWireNumber) });
