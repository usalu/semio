/** 📐️ `change-taper-inner-radius` wire twin: the leaf payload `ChangeTaperInnerRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperInnerRadius {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperInnerRadius: NormWireReader<ChangeTaperInnerRadius> = normWireObject<ChangeTaperInnerRadius>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
