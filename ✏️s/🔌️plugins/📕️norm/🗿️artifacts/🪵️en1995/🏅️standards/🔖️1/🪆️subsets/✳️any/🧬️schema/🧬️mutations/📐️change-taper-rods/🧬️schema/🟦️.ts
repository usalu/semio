/** 📐️ `change-taper-rods` wire twin: the leaf payload `ChangeTaperRods`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperRods {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperRods: NormWireReader<ChangeTaperRods> = normWireObject<ChangeTaperRods>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
