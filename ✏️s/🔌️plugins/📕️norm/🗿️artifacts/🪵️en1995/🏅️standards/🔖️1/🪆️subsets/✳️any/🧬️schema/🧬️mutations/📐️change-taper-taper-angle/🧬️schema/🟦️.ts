/** 📐️ `change-taper-taper-angle` wire twin: the leaf payload `ChangeTaperTaperAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperTaperAngle {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperTaperAngle: NormWireReader<ChangeTaperTaperAngle> = normWireObject<ChangeTaperTaperAngle>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
