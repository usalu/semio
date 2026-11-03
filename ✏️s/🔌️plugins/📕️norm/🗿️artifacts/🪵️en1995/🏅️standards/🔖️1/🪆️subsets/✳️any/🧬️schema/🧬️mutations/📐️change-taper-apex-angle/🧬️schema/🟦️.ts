/** 📐️ `change-taper-apex-angle` wire twin: the leaf payload `ChangeTaperApexAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperApexAngle {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperApexAngle: NormWireReader<ChangeTaperApexAngle> = normWireObject<ChangeTaperApexAngle>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
