/** 🔁️ `change-fatigue-band-delta-sigma` wire twin: the leaf payload `ChangeFatigueBandDeltaSigma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueBandDeltaSigma {
  id: string;
  elementId: string;
  newDeltaSigma: number;
}

export const parseChangeFatigueBandDeltaSigma: NormWireReader<ChangeFatigueBandDeltaSigma> = normWireObject<ChangeFatigueBandDeltaSigma>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newDeltaSigma: normWireRequired(normWireNumber) });
