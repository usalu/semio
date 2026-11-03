/** ⭕ `change-silo-diameter` wire twin: the leaf payload `ChangeSiloDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloDiameter {
  newSiloDiameter: number;
}

export const parseChangeSiloDiameter: NormWireReader<ChangeSiloDiameter> = normWireObject<ChangeSiloDiameter>({ newSiloDiameter: normWireRequired(normWireRange(normWireNumber, {"minimum":0.5})) });
