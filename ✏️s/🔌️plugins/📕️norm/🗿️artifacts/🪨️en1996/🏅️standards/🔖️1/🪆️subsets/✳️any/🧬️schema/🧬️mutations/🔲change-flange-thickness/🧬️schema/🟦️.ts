/** 🔲 `change-flange-thickness` wire twin: the leaf payload `ChangeFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFlangeThickness {
  wallId: string;
  id: string;
  newThicknessM: number;
}

export const parseChangeFlangeThickness: NormWireReader<ChangeFlangeThickness> = normWireObject<ChangeFlangeThickness>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
