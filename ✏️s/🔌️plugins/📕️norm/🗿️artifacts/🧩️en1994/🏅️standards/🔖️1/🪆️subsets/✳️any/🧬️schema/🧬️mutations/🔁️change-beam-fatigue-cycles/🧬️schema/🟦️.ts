/** 🔁️ `change-beam-fatigue-cycles` wire twin: the leaf payload `ChangeBeamFatigueCycles`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamFatigueCycles {
  id: string;
  newNCycles: number;
}

export const parseChangeBeamFatigueCycles: NormWireReader<ChangeBeamFatigueCycles> = normWireObject<ChangeBeamFatigueCycles>({ id: normWireRequired(normWireString), newNCycles: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
