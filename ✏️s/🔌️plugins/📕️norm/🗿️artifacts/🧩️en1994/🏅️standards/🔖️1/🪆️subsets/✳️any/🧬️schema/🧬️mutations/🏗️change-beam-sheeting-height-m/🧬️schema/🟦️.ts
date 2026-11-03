/** 🏗️ `change-beam-sheeting-height-m` wire twin: the leaf payload `ChangeBeamSheetingHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSheetingHeightM {
  id: string;
  newHeightM: number;
}

export const parseChangeBeamSheetingHeightM: NormWireReader<ChangeBeamSheetingHeightM> = normWireObject<ChangeBeamSheetingHeightM>({ id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
