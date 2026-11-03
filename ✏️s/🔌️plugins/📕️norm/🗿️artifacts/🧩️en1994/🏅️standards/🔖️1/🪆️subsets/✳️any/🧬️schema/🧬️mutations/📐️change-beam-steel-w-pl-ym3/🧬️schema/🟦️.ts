/** 📐️ `change-beam-steel-w-pl-ym3` wire twin: the leaf payload `ChangeBeamSteelWPlYM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelWPlYM3 {
  id: string;
  newWPlYM3: number;
}

export const parseChangeBeamSteelWPlYM3: NormWireReader<ChangeBeamSteelWPlYM3> = normWireObject<ChangeBeamSteelWPlYM3>({ id: normWireRequired(normWireString), newWPlYM3: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
