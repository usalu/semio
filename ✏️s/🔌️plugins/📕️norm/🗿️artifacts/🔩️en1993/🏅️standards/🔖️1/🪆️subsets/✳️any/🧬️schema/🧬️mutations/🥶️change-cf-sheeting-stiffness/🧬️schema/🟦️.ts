/** 🥶️ `change-cf-sheeting-stiffness` wire twin: the leaf payload `ChangeCfSheetingStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfSheetingStiffness {
  id: string;
  newSheetingStiffness: number;
}

export const parseChangeCfSheetingStiffness: NormWireReader<ChangeCfSheetingStiffness> = normWireObject<ChangeCfSheetingStiffness>({ id: normWireRequired(normWireString), newSheetingStiffness: normWireRequired(normWireNumber) });
