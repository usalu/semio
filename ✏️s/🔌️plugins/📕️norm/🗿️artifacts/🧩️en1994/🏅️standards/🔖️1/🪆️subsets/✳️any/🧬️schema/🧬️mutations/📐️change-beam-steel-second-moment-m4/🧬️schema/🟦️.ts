/** 📐️ `change-beam-steel-second-moment-m4` wire twin: the leaf payload `ChangeBeamSteelSecondMomentM4`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelSecondMomentM4 {
  id: string;
  newIYM4: number;
}

export const parseChangeBeamSteelSecondMomentM4: NormWireReader<ChangeBeamSteelSecondMomentM4> = normWireObject<ChangeBeamSteelSecondMomentM4>({ id: normWireRequired(normWireString), newIYM4: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
