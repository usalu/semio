/** 📏️ `change-member-margin-shift` wire twin: the leaf payload `ChangeMemberMarginShift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMarginShift {
  id: string;
  newMarginShift: number;
}

export const parseChangeMemberMarginShift: NormWireReader<ChangeMemberMarginShift> = normWireObject<ChangeMemberMarginShift>({ id: normWireRequired(normWireString), newMarginShift: normWireRequired(normWireNumber) });
