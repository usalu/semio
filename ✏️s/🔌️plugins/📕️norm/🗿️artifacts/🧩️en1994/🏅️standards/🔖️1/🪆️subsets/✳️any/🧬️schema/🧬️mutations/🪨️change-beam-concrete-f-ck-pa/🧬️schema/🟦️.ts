/** 🪨️ `change-beam-concrete-f-ck-pa` wire twin: the leaf payload `ChangeBeamConcreteFCkPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamConcreteFCkPa {
  id: string;
  newConcreteFCkPa: number;
}

export const parseChangeBeamConcreteFCkPa: NormWireReader<ChangeBeamConcreteFCkPa> = normWireObject<ChangeBeamConcreteFCkPa>({ id: normWireRequired(normWireString), newConcreteFCkPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
