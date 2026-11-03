/** 🌬️ `change-qp-wind` wire twin: the leaf payload `ChangeQPWind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeQPWind {
  wallId: string;
  id: string;
  newQPWindPa: number;
}

export const parseChangeQPWind: NormWireReader<ChangeQPWind> = normWireObject<ChangeQPWind>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQPWindPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
