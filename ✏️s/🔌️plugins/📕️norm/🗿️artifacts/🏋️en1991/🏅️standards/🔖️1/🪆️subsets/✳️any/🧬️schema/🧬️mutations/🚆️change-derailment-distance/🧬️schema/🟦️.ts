/** 🚆 `change-derailment-distance` wire twin: the leaf payload `ChangeDerailmentDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDerailmentDistance {
  id: string;
  newDistance: number;
}

export const parseChangeDerailmentDistance: NormWireReader<ChangeDerailmentDistance> = normWireObject<ChangeDerailmentDistance>({ id: normWireRequired(normWireString), newDistance: normWireRequired(normWireNumber) });
