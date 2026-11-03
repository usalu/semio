/** 📐️ `change-taper-lamella` wire twin: the leaf payload `ChangeTaperLamella`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperLamella {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperLamella: NormWireReader<ChangeTaperLamella> = normWireObject<ChangeTaperLamella>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
