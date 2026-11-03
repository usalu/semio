/** 🔁️ `change-fatigue-band-cycles` wire twin: the leaf payload `ChangeFatigueBandCycles`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueBandCycles {
  id: string;
  elementId: string;
  newCycles: number;
}

export const parseChangeFatigueBandCycles: NormWireReader<ChangeFatigueBandCycles> = normWireObject<ChangeFatigueBandCycles>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newCycles: normWireRequired(normWireNumber) });
